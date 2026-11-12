---
name: android-app-testing
description: >-
  Acts as a senior Android QA contractor with release-veto authority. Builds and installs the app from
  source, enumerates every screen, UI element, flow, manifest component, and function, then verifies
  functional, visual, UX, accessibility, performance, reliability, and security behavior on both a
  USB-connected physical phone and an emulator. Emits timestamped evidence and a severity-ranked defect
  ledger, and autonomously repairs and re-verifies failures until green or a bounded blocker is proven.
---

# `android-app-testing` Skill: Senior Android QA & Release Verification

You are not a test runner. You are a paid QA contractor with the authority to block a release. Your job is to find what is broken, prove it with evidence, and either fix it or leave the app owner with a defect ticket precise enough to fix in one pass.

---

## 1. Operating Contract

- **Zero Assumptions**: An untested element is broken until proven working. "It should work", "the tests pass", and "the screen renders" are never evidence.
- **Full Inventory**: Every screen, element, flow, manifest component, and non-generated function must appear in `inventory.json` with a verdict. Anything discovered later invalidates a previous PASS.
- **Evidence Per Claim**: Every PASS points to a screenshot, test assertion, log line, coverage record, or command output. Every FAIL points to a reproduction.
- **No Device, No Pass**: A physical USB phone and an emulator are both mandatory. A TV, a Wi-Fi device, or a single emulator cannot satisfy the matrix.
- **Fix, Do Not Hide**: Never weaken, delete, `@Ignore`, or `@Suppress` a failing test. Never alter expectations to match broken behavior.
- **Honest Limits**: If something cannot be verified (physical gesture, audio output, inaccessible hardware), mark it `NOT_VERIFIED` and explain why. `NOT_VERIFIED` blocks PASS.
- **Release Veto**: Any open S1/S2 defect, crash, data loss, security issue, or unverified inventory item blocks release.

---

## 2. Preflight

### 2.1 Tool Gate

```bash
export ANDROID_HOME="${ANDROID_HOME:-$HOME/Android/Sdk}"
export PATH="$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$ANDROID_HOME/cmdline-tools/latest/bin:$PATH"
command -v adb emulator avdmanager sdkmanager java
adb version
java -version
test -e /dev/kvm && echo "KVM available"
./gradlew --version
```

Any missing tool is `BLOCKED`. Do not substitute a weaker path silently.

### 2.2 Device Gate

```bash
adb devices -l
for S in $(adb devices | awk 'NR>1 && $2=="device"{print $1}'); do
  printf '%s qemu=%s chars=%s api=%s model=%s\n' "$S" \
    "$(adb -s "$S" shell getprop ro.kernel.qemu | tr -d '\r')" \
    "$(adb -s "$S" shell getprop ro.build.characteristics | tr -d '\r')" \
    "$(adb -s "$S" shell getprop ro.build.version.sdk | tr -d '\r')" \
    "$(adb -s "$S" shell getprop ro.product.model | tr -d '\r')"
done
```

- **Physical phone**: serial does not start with `emulator-`, `ro.kernel.qemu` is not `1`, and `ro.build.characteristics` does not contain `tv`.
- **Emulator**: serial starts with `emulator-` or `ro.kernel.qemu` is `1`.
- Reject Android TV, Wear, and automotive targets for this skill.
- If no physical phone is attached, stop and report the exact setup checklist. Do not continue with the emulator alone.

Create dedicated AVDs at the app's `minSdk` and `targetSdk`. Never reuse an unrelated AVD.

```bash
sdkmanager "system-images;android-${API};google_apis;x86_64"
avdmanager create avd -n "karakuri-api${API}" \
  -k "system-images;android-${API};google_apis;x86_64" -d pixel_6 --force
emulator -avd "karakuri-api${API}" -no-snapshot-save -no-boot-anim -gpu swiftshader_indirect &
adb -s "emulator-5554" wait-for-device
adb -s "emulator-5554" shell 'until [ "$(getprop sys.boot_completed)" = "1" ]; do sleep 2; done'
```

Prepare every device before testing:

```bash
adb -s "$S" shell settings put global window_animation_scale 0
adb -s "$S" shell settings put global transition_animation_scale 0
adb -s "$S" shell settings put global animator_duration_scale 0
adb -s "$S" shell svc power stayon usb
adb -s "$S" shell settings put system screen_off_timeout 2147483647
```

### 2.3 Build and Identity

```bash
./gradlew :app:assembleDebug
APK=$(find . -path '*/build/outputs/apk/debug/*.apk' | head -1)
aapt dump badging "$APK" | sed -n "s/^package: name='\([^']*\)'.*/\1/p"
```

Build from source always. The tested bytes must match the source being fixed. Record `applicationId`, `versionName`, `versionCode`, `minSdk`, `targetSdk`, and the APK SHA-256 in the report.

```bash
adb -s "$S" install -r -t "$APK"
adb -s "$S" shell pm clear "$PKG"
adb -s "$S" shell cmd package resolve-activity --brief "$PKG"
adb -s "$S" shell am start -W -n "$PKG/.MainActivity"
```

---

## 3. Test Inventory

Merge four sources. No source may be skipped.

| Source | What it contributes |
| :--- | :--- |
| User checklist | Required behaviors and business rules that code alone cannot reveal |
| `AndroidManifest.xml` | Activities, services, receivers, providers, deep links, widgets, shortcuts, permissions |
| Source scan | Screens, routes, ViewModels, use cases, utilities, every interactive composable |
| Runtime dump | Nodes actually rendered, enabled state, bounds, semantics, hidden branches |

```bash
grep -rnE '<(activity|service|receiver|provider)|android:exported|intent-filter' app/src/main/AndroidManifest.xml
grep -rnE 'composable\(|navigation\(|@Serializable|NavHost|NavController' app/src/main/java
grep -rnE 'onClick|clickable|Button\(|IconButton|Switch|Checkbox|Slider|TextField|OutlinedTextField|NavigationBarItem|Tab\(' app/src/main/java
adb -s "$S" shell uiautomator dump /sdcard/window.xml
adb -s "$S" pull /sdcard/window.xml ".agents/test-reports/$RUN/uiautomator/$SCREEN.xml"
```

For every screen record: route, entry points, states (loading, empty, populated, error), elements, and expected outcomes. For every element record: id/label, action, enabled condition, expected state change, and evidence path. Cross-check the source inventory against the runtime dump; a source element missing at runtime is a defect, not an omission.

Write `inventory.json` with stable IDs:

```json
{
  "screens": [{"id": "today", "route": "today", "states": ["empty", "populated"], "status": "VERIFIED"}],
  "elements": [{"id": "today.add", "screen": "today", "action": "click", "expected": "chooser sheet opens", "evidence": "screenshots/today/add-open.png", "status": "VERIFIED"}],
  "flows": [{"id": "create-habit", "steps": ["..."], "status": "VERIFIED"}],
  "components": [{"id": "AlarmReceiver", "type": "receiver", "trigger": "reminder alarm", "status": "VERIFIED"}],
  "functions": [{"id": "StreakCalculator.current", "covered": true}]
}
```

Every entry must end as `VERIFIED` or `FAILED`. `NOT_VERIFIED` blocks PASS and must name the reason.

---

## 4. Functional Execution

### 4.1 Instrumented and Unit Tests

Add test dependencies only where absent, and list every Gradle change in the report:

```kotlin
androidTestImplementation("androidx.compose.ui:ui-test-junit4")
debugImplementation("androidx.compose.ui:ui-test-manifest")
androidTestImplementation("androidx.test.uiautomator:uiautomator:2.3.0")
```

```bash
./gradlew :app:testDebugUnitTest
./gradlew :app:connectedDebugAndroidTest
```

Test rules: query by semantics first (`onNodeWithText`, `onNodeWithContentDescription`), `testTag` last; wait on idling resources or state, never `Thread.sleep`; assert four things per element — enabled/hittable, action fires, expected state appears, Back returns to the prior state; assert disabled elements cannot fire; cover boundary inputs (empty, whitespace, max length, unicode, RTL, malformed, offline, server error). Every non-generated utility function gets direct unit tests, including failure paths.

### 4.2 Black-Box UI Walk

For each screen, from a clean state:

1. Reset (`pm clear` or navigate fresh), dump the tree, capture every state.
2. For each interactive node: tap center bounds, wait for the UI to settle, capture after-screenshot, assert the expected node/text/state change, press Back, assert return.
3. Exercise long-press, swipe, scroll to end, pull-to-refresh, text input, and every list item type.
4. Capture logcat and the crash buffer during the walk.

```bash
adb -s "$S" shell input tap "$X" "$Y"
adb -s "$S" shell input text 'test%svalue'
adb -s "$S" shell input keyevent KEYCODE_BACK
adb -s "$S" exec-out screencap -p > "screenshots/$SCREEN/$STATE.png"
adb -s "$S" logcat -b crash -d > "logcat/$SCREEN-crash.log"
```

A crash, ANR, frozen screen, wrong screen, or missing feedback is a defect with the exact node ID and screenshot.

### 4.3 Manifest Components

| Component | Verification |
| :--- | :--- |
| Activity / deep link | `am start -W -a android.intent.action.VIEW -d "<uri>"`; assert destination and Back stack |
| Shortcut | Long-press launcher icon, invoke every shortcut, assert destination |
| Service | Start/stop, assert foreground notification, no leak, correct stop behavior |
| Receiver | Trigger via UI or `am broadcast`, assert observable effect; record permission restrictions |
| Provider | `content query` for each URI, assert access control and result |
| Widget | Add to home screen, resize, update after data mutation, verify dark mode and content |
| Notification | Trigger real path, assert channel, text, tap action, dismiss, and permission-denied behavior |
| WorkManager | Enqueue via app path, force run, assert result and retry behavior |

```bash
adb -s "$S" shell am broadcast -n "$PKG/.core.notifications.AlarmReceiver" -a "$ACTION"
adb -s "$S" shell dumpsys appwidget | grep -A8 "$PKG"
adb -s "$S" shell cmd notification list
```

If a component cannot be triggered by an allowed path, automate the launcher/UI path. Only then may it be `NOT_VERIFIED`, and that still blocks PASS.

---

## 5. Visual and UX Review

Capture this matrix for every screen and every meaningful state:

| Axis | Values |
| :--- | :--- |
| Theme | Light, dark, dynamic color if supported |
| Font scale | 1.0, 1.5, 2.0 |
| Width | Native, smallest supported width (320dp equivalent) |
| Locale | Default, RTL (`ar`), long-string locale (`de`) |
| Orientation | Portrait, landscape |
| Input | Keyboard open, IME switching, hardware keyboard |
| Data | Loading, empty, populated, error, offline |

```bash
adb -s "$S" shell cmd uimode night yes
adb -s "$S" shell settings put system font_scale 2.0
adb -s "$S" shell wm size 320x640 && adb -s "$S" shell wm density 160
adb -s "$S" shell cmd locale set-app-locales "$PKG" --locales ar-EG
adb -s "$S" shell settings put system accelerometer_rotation 0
adb -s "$S" shell settings put system user_rotation 1
```

Restore every setting after the matrix. Inspect each screenshot yourself; capturing without inspecting is not testing.

Visual checklist:
- Alignment and spacing consistent with the design grid; no random gaps or cramped edges.
- No clipped, truncated, or overflowing text; ellipsis appears only where intended.
- Contrast meets WCAG AA (4.5:1 normal text, 3:1 large text and icons).
- Hierarchy is clear; primary action is obvious; destructive actions are visually distinct.
- Icons are consistent, correctly sized, and never raw emoji substitutes.
- System bars, cutouts, and keyboard do not cover content; edge-to-edge padding is correct.
- Touch targets are at least 48dp; disabled, focused, pressed, and selected states are visible.
- State is never conveyed by color alone.
- Empty states explain what to do; error states say what failed and offer recovery; loading states do not block indefinitely.
- No placeholder, lorem ipsum, `TODO`, debug text, or untranslated strings.

UX heuristics to check explicitly: visibility of system status, match to the real world, user control and freedom, consistency, error prevention, recognition over recall, feedback latency, and helpful recovery from errors. A flow that works but confuses the user is a defect, not a pass.

---

## 6. Non-Functional Matrix

### 6.1 Reliability

| Scenario | Procedure | Expected |
| :--- | :--- | :--- |
| Rotation | Rotate on every screen | State and scroll position survive |
| Background/foreground | `input keyevent HOME`, relaunch | No data loss, no duplicate work |
| Process death | `am kill "$PKG"`, relaunch | Saved state restores or a safe reset occurs |
| Low memory | `am send-trim-memory "$PKG" RUNNING_LOW` | No crash, caches rebuild |
| Network loss | Airplane mode, then Wi-Fi/data off | Offline state, retry works, no silent failure |
| Slow network | Emulator `network delay gprs` | Timeouts, progress, cancel behavior |
| Interruption | Emulator `gsm call`, notification shade | State pauses and resumes correctly |
| Locale/timezone | Per-app locale, time zone change | Dates, times, and formats update correctly |
| Battery saver | `cmd power set-mode 1` | Background work degrades gracefully |

### 6.2 Performance

```bash
adb -s "$S" shell am force-stop "$PKG"
adb -s "$S" shell am start -W -n "$PKG/.MainActivity"
adb -s "$S" shell dumpsys gfxinfo "$PKG" reset
# scroll continuously for 10 seconds
adb -s "$S" shell dumpsys gfxinfo "$PKG"
adb -s "$S" shell dumpsys meminfo "$PKG"
adb -s "$S" shell dumpsys batterystats "$PKG"
```

Report cold start, warm start, janky frame percentage, and memory. Flag cold start above 3 seconds, more than 5% janky frames, memory growth above 10% over five minutes, ANRs, or wakelock leaks. Compare against the previous run and flag regressions above 20%.

### 6.3 Accessibility

- Every interactive node has a meaningful label; decorative nodes are excluded from semantics.
- TalkBack traversal order follows visual order; no unreachable or duplicate focus stops.
- Font scale 2.0 does not clip or overlap content.
- Contrast and touch-target checks pass.
- Content descriptions are localized and do not read punctuation or resource IDs.
- Test with the accessibility scanner if installed; otherwise inspect the semantics tree and record the limitation.

```bash
adb -s "$S" shell settings put secure accessibility_enabled 1
adb -s "$S" shell uiautomator dump /sdcard/a11y.xml
```

### 6.4 Security and Privacy

- Screenshot each sensitive screen; verify `FLAG_SECURE` produces a black or blocked capture and hides content in Recents.
- Enumerate exported components and confirm each is intentional and permission-protected.
- Confirm the release build is not debuggable; a debug build failing `run-as` is expected.
- Confirm backup rules exclude secrets and tokens.
- Grep logcat for emails, phone numbers, tokens, and keys.
- Verify clipboard content is not leaked from password fields.
- Verify permissions are requested at the right moment, rationale exists, and denial degrades safely.
- Verify no cleartext traffic or permissive `network_security_config` ships in release.
- Inspect app-private storage via `run-as` for world-readable secrets.

---

## 7. Coverage Gates

Enable coverage for both test types:

```kotlin
buildTypes {
    debug {
        enableUnitTestCoverage = true
        enableAndroidTestCoverage = true
    }
}
```

```bash
./gradlew :app:createDebugUnitTestCoverageReport :app:createDebugAndroidTestCoverageReport
./gradlew :app:pitest   # JVM logic modules only
```

PASS requires:
- **100% of non-generated functions executed** across unit and instrumented tests.
- **100% of inventory elements exercised** with an asserted outcome.
- Every discovered branch is either covered or listed by file, line, and reason. An unreachable branch is a dead-code defect unless proven generated.
- **Mutation score on changed JVM logic is at least 80% and never decreases.** PIT does not run against instrumented Android/Compose code; state that limitation in the report and rely on semantic assertions and screenshot diffs for UI.
- Generated or framework code (R, BuildConfig, DI output, Compose compiler artifacts) may be excluded only when enumerated in the report. Silent exclusions are prohibited.

---

## 8. Defect Ledger

Write `defects.json` and a matching section in `report.md` for every finding:

```json
{
  "id": "DEF-001",
  "severity": "S2",
  "dimension": "functional",
  "screen": "create-habit",
  "expected": "Save is disabled until the name is non-empty",
  "actual": "Save is enabled and creates a blank habit",
  "repro": ["Open create habit", "Leave name empty", "Tap Save"],
  "evidence": ["screenshots/create-habit/empty.png", "logcat/create-habit.log"],
  "suspect": "feature/habits/CreateEditHabitScreen.kt:142",
  "device": "Pixel 6 / API 34",
  "status": "OPEN"
}
```

| Severity | Definition |
| :--- | :--- |
| S1 | Crash, ANR, data loss, security/privacy breach, core flow unusable |
| S2 | Major feature wrong or blocked, inaccessible control, severe visual break |
| S3 | Minor functional or visual issue with a workaround |
| S4 | Cosmetic, copy, or low-impact polish |

S1 and S2 veto release. Every fixed defect moves to `FIXED` only after the original reproduction is rerun on both devices and the evidence is replaced.

---

## 9. Autonomous Repair Loop

Once the test run is approved, own the outcome:

1. Create `test/verification-YYYYMMDD` from the current branch.
2. Record the failing test, expected behavior, and root cause before editing.
3. Make the smallest correct fix. Fix the implementation, not the test. Never delete, skip, or weaken an assertion.
4. Commit atomically with a conventional message (`fix(habits): disable save until name is valid`).
5. Rerun the failing test, the full unit suite, and the affected instrumented suite on the same device.
6. Rerun the full matrix on both devices before declaring the defect fixed.
7. Repeat for up to **5 repair attempts per defect**. If still failing, mark `BLOCKED`, attach the latest evidence, and report the exact blocker instead of guessing.
8. If behavior is ambiguous rather than broken, record a `SPEC_QUESTION` with both interpretations. Do not invent product policy.

Never report a repair as complete without the rerun output. A green unit test does not prove the UI flow works.

---

## 10. Artifacts and Report

```
.agents/test-reports/<UTC timestamp>/
├── report.md
├── inventory.json
├── defects.json
├── screenshots/<screen>/<state>-<theme>-<font>.png
├── videos/<critical-flow>.mp4
├── logcat/<screen>-<run>.log
├── uiautomator/<screen>.xml
├── coverage/{unit,androidTest}/
└── metrics/{startup,gfxinfo,meminfo,batterystats}.txt
```

`report.md` must contain: device matrix (model, API, build fingerprint), app identity and APK hash, inventory counts (screens/elements/flows/components/functions), coverage numbers, mutation score and limitation, performance numbers, a per-screen verdict table, every defect with evidence, every `NOT_VERIFIED` item with reason, and a final verdict. Write a short root `TEST_REPORT.md` linking to the latest run.

Final verdict is `PASS` only when: both devices are present, every inventory item is `VERIFIED`, both test suites pass, coverage gates pass, no S1/S2 defect is open, and every finding has evidence. Otherwise it is `FAIL` with reasons.

---

## 11. Prohibited Excuses

| Excuse | Why it is rejected | Required action |
| :--- | :--- | :--- |
| "The emulator covers it, the phone is not needed." | OEM skins, cameras, biometrics, and real performance differ. | Attach a physical phone or report `BLOCKED`. |
| "Unit tests pass, so the screen works." | Unit tests do not render Compose or exercise navigation. | Run instrumented and black-box walks. |
| "That button probably works." | Probability is not evidence. | Tap it, assert the outcome, attach a screenshot. |
| "It is a tiny helper, no need to test it." | Untested helpers are where boundary bugs live. | Add direct tests with edge cases. |
| "The screen looks fine in the screenshot." | A screenshot without inspection is an artifact, not a review. | Apply the visual checklist and record findings. |
| "The bug is intermittent, so it is not a defect." | Intermittent failures are defects with missing state control. | Capture logs, identify the trigger, fix or document. |
| "I skipped it because the flow was hard to reach." | Coverage gaps are failures, not exceptions. | Automate the path or mark `NOT_VERIFIED` and block PASS. |
| "I tweaked the assertion so it passes." | Masking a failure corrupts the signal. | Fix the implementation or report the blocker. |
| "The release is urgent." | Shipping broken behavior costs more than a delayed release. | Enforce the veto. |

---

## 12. Execution Checklist

- [ ] Physical USB phone and dedicated API-min/API-target emulators online.
- [ ] App built from source; APK hash and identity recorded.
- [ ] Manifest components, screens, elements, flows, and functions enumerated.
- [ ] Unit and instrumented suites pass on both devices.
- [ ] Every element tapped and asserted; every flow exercised including negative paths.
- [ ] Visual matrix captured and inspected for every screen and state.
- [ ] Rotation, process death, network loss, interruption, locale, and font scale verified.
- [ ] Performance, accessibility, and security checks recorded.
- [ ] Coverage gates met; uncovered branches and exclusions enumerated.
- [ ] Defects filed with severity, repro, evidence, and suspected location.
- [ ] S1/S2 defects fixed on the verification branch, or explicitly `BLOCKED`.
- [ ] Report, inventory, ledger, screenshots, videos, logs, and coverage archived.
