from dataclasses import dataclass
from enum import Enum
from test_results_parser import escape_failure_message, shorten_file_paths, build_message

def test_escape_failure_message():
    with open('./tests/samples/windows.junit.xml') as f:
        failure_message = f.read()
    res = escape_failure_message(failure_message)

    assert res == """Error: expect(received).toBe(expected) // Object.is equality<br><br>Expected: 4<br>Received: 5<br>at Object.&amp;lt;anonymous&amp;gt;<br>(/Users/user/dev/repo/demo/calculator/calculator.test.ts:5:26)<br>at Promise.then.completed<br>(/Users/user/dev/repo/node\_modules/jest-circus/build/utils.js:298:28)<br>at new Promise (&amp;lt;anonymous&amp;gt;)<br>at callAsyncCircusFn<br>(/Users/user/dev/repo/node\_modules/jest-circus/build/utils.js:231:10)<br>at \_callCircusTest<br>(/Users/user/dev/repo/node\_modules/jest-circus/build/run.js:316:40)<br>at processTicksAndRejections (node:internal/process/task\_queues:95:5)<br>at \_runTest<br>(/Users/user/dev/repo/node\_modules/jest-circus/build/run.js:252:3)<br>at \_runTestsForDescribeBlock<br>(/Users/user/dev/repo/node\_modules/jest-circus/build/run.js:126:9)<br>at run<br>(/Users/user/dev/repo/node\_modules/jest-circus/build/run.js:71:3)<br>at runAndTransformResultsToJestFormat<br>(/Users/user/dev/repo/node\_modules/jest-circus/build/legacy-code-todo-rewrite/jestAdapterInit.js:122:21)<br>at jestAdapter<br>(/Users/user/dev/repo/node\_modules/jest-circus/build/legacy-code-todo-rewrite/jestAdapter.js:79:19)<br>at runTestInternal<br>(/Users/user/dev/repo/node\_modules/jest-runner/build/runTest.js:367:16)<br>at runTest<br>(/Users/user/dev/repo/node\_modules/jest-runner/build/runTest.js:444:34)"""

def test_shorten_file_paths():
    with open('./tests/samples/windows.junit.xml') as f:
        failure_message = f.read()

    res = shorten_file_paths(failure_message)

    assert res == """Error: expect(received).toBe(expected) // Object.is equality

Expected: 4
Received: 5
at Object.&lt;anonymous&gt;
(.../demo/calculator/calculator.test.ts:5:26)
at Promise.then.completed
(.../jest-circus/build/utils.js:298:28)
at new Promise (&lt;anonymous&gt;)
at callAsyncCircusFn
(.../jest-circus/build/utils.js:231:10)
at _callCircusTest
(.../jest-circus/build/run.js:316:40)
at processTicksAndRejections (node:internal/process/task_queues:95:5)
at _runTest
(.../jest-circus/build/run.js:252:3)
at _runTestsForDescribeBlock
(.../jest-circus/build/run.js:126:9)
at run
(.../jest-circus/build/run.js:71:3)
at runAndTransformResultsToJestFormat
(.../build/legacy-code-todo-rewrite/jestAdapterInit.js:122:21)
at jestAdapter
(.../build/legacy-code-todo-rewrite/jestAdapter.js:79:19)
at runTestInternal
(.../jest-runner/build/runTest.js:367:16)
at runTest
(.../jest-runner/build/runTest.js:444:34)"""

def test_shorten_and_escape_failure_message():
    with open('./tests/samples/windows.junit.xml') as f:
        failure_message = f.read()

    partial_res = shorten_file_paths(failure_message)
    res = escape_failure_message(partial_res)
   
    assert res == """Error: expect(received).toBe(expected) // Object.is equality<br><br>Expected: 4<br>Received: 5<br>at Object.&amp;lt;anonymous&amp;gt;<br>(.../demo/calculator/calculator.test.ts:5:26)<br>at Promise.then.completed<br>(.../jest-circus/build/utils.js:298:28)<br>at new Promise (&amp;lt;anonymous&amp;gt;)<br>at callAsyncCircusFn<br>(.../jest-circus/build/utils.js:231:10)<br>at \_callCircusTest<br>(.../jest-circus/build/run.js:316:40)<br>at processTicksAndRejections (node:internal/process/task\_queues:95:5)<br>at \_runTest<br>(.../jest-circus/build/run.js:252:3)<br>at \_runTestsForDescribeBlock<br>(.../jest-circus/build/run.js:126:9)<br>at run<br>(.../jest-circus/build/run.js:71:3)<br>at runAndTransformResultsToJestFormat<br>(.../build/legacy-code-todo-rewrite/jestAdapterInit.js:122:21)<br>at jestAdapter<br>(.../build/legacy-code-todo-rewrite/jestAdapter.js:79:19)<br>at runTestInternal<br>(.../jest-runner/build/runTest.js:367:16)<br>at runTest<br>(.../jest-runner/build/runTest.js:444:34)"""


def test_escape_failure_message_happy_path():
    failure_message = "\"'<>&\r\n"
    res = escape_failure_message(failure_message)
    assert res == "&amp;quot;&amp;apos;&amp;lt;&amp;gt;&amp;<br>"

def test_escape_failure_message_slash_in_message():
    failure_message = "\\ \\n \n"
    res = escape_failure_message(failure_message)
    assert res == "\\ \\n <br>"

def test_shorten_file_paths_short_path():
    failure_message = "short/file/path.txt"
    res = shorten_file_paths(failure_message)
    assert res == failure_message

def test_shorten_file_paths_long_path():
    failure_message = "very/long/file/path/should/be/shortened.txt"
    res = shorten_file_paths(failure_message)
    assert res == ".../should/be/shortened.txt"

def test_shorten_file_paths_long_path_leading_slash():
    failure_message = "/very/long/file/path/should/be/shortened.txt"
    res = shorten_file_paths(failure_message)
    assert res == ".../should/be/shortened.txt"

def test_build_message():

    class FlakeSymptomType(Enum):
        FAILED_IN_DEFAULT_BRANCH = "failed_in_default_branch"
        CONSECUTIVE_DIFF_OUTCOMES = "consecutive_diff_outcomes"
        UNRELATED_MATCHING_FAILURES = "unrelated_matching_failures"


    @dataclass
    class TestResultsNotificationFailure:
        failure_message: str = ""
        testsuite: str = ""
        name: str = ""
        flags: list[str] = lambda: list()
        test_id: str = ""


    @dataclass
    class TestResultsNotificationFlake:
        flake_type: list[FlakeSymptomType] = lambda: list()
        is_new_flake: bool = False


    @dataclass
    class TestResultsNotificationPayload:
        failed: int = 0
        passed: int = 0
        skipped: int = 0
        failures: list[TestResultsNotificationFailure] = lambda: list()
        flaky_tests: dict[str, TestResultsNotificationFlake] | None = None

    
    fail = TestResultsNotificationFailure(failure_message="hello world", testsuite="hello", name="world", flags=["i", "am"], test_id="you are")
    fail2 = TestResultsNotificationFailure(failure_message="foo", testsuite="bar", name="world", flags=["i", "am"], test_id="other test")


    flake = TestResultsNotificationFlake(flake_type=[FlakeSymptomType.FAILED_IN_DEFAULT_BRANCH, FlakeSymptomType.CONSECUTIVE_DIFF_OUTCOMES], is_new_flake=False)
    flake.flake_type = [FlakeSymptomType.FAILED_IN_DEFAULT_BRANCH, FlakeSymptomType.CONSECUTIVE_DIFF_OUTCOMES]
    flake.is_new_flake = False

    payload = TestResultsNotificationPayload(
        failed=1,
        passed=2,
        skipped=3,
        failures=[fail, fail2],
        flaky_tests={
            "you are": flake
        }
    )


    res = build_message(payload)

    assert res == """### :x: Failed Test Results: 
Completed 6 tests with **`1 failed`**, 2 passed and 3 skipped.
<details><summary>View the full list of failed tests</summary>

| **Test Description** | **Failure message** |
| :-- | :-- |
| :snowflake::card_index_dividers: **Known Flaky Test**<br><pre>Testsuite:<br>hello<br><br>Test name:<br>world<br>**Flags**:<br>- i<br>- am<br></pre> | :snowflake: :card_index_dividers: **Failure on default branch**<br>:snowflake: :card_index_dividers: **Differing outcomes on the same commit**<br><pre>hello world</pre> |
| <pre>Testsuite:<br>bar<br><br>Test name:<br>world<br>**Flags**:<br>- i<br>- am<br></pre> | <pre>foo</pre> |"""
