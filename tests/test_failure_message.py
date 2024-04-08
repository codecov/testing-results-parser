import pytest 
from test_results_parser import escape_failure_message_pystring, shorten_file_paths_pystring, escape_failure_message_rust_string, shorten_file_paths_rust_string

@pytest.mark.parametrize("escape_failure_message", [escape_failure_message_pystring, escape_failure_message_rust_string])
def test_escape_failure_message(escape_failure_message):
    with open('./tests/windows.junit.xml') as f:
        failure_message = f.read()
    res = escape_failure_message(failure_message)

    assert res == """Error: expect(received).toBe(expected) // Object.is equality<br><br>Expected: 4<br>Received: 5<br>at Object.&amp;lt;anonymous&amp;gt;<br>(/Users/user/dev/repo/demo/calculator/calculator.test.ts:5:26)<br>at Promise.then.completed<br>(/Users/user/dev/repo/node_modules/jest-circus/build/utils.js:298:28)<br>at new Promise (&amp;lt;anonymous&amp;gt;)<br>at callAsyncCircusFn<br>(/Users/user/dev/repo/node_modules/jest-circus/build/utils.js:231:10)<br>at _callCircusTest<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:316:40)<br>at processTicksAndRejections (node:internal/process/task_queues:95:5)<br>at _runTest<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:252:3)<br>at _runTestsForDescribeBlock<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:126:9)<br>at run<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:71:3)<br>at runAndTransformResultsToJestFormat<br>(/Users/user/dev/repo/node_modules/jest-circus/build/legacy-code-todo-rewrite/jestAdapterInit.js:122:21)<br>at jestAdapter<br>(/Users/user/dev/repo/node_modules/jest-circus/build/legacy-code-todo-rewrite/jestAdapter.js:79:19)<br>at runTestInternal<br>(/Users/user/dev/repo/node_modules/jest-runner/build/runTest.js:367:16)<br>at runTest<br>(/Users/user/dev/repo/node_modules/jest-runner/build/runTest.js:444:34)"""

@pytest.mark.parametrize("shorten_file_paths", [shorten_file_paths_pystring, shorten_file_paths_rust_string])
def test_shorten_file_paths(shorten_file_paths):
    with open('./tests/windows.junit.xml') as f:
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

@pytest.mark.parametrize("shorten_file_paths,escape_failure_message", [(shorten_file_paths_pystring, escape_failure_message_pystring),(shorten_file_paths_rust_string, escape_failure_message_rust_string)])
def test_shorten_and_escape_failure_message(shorten_file_paths, escape_failure_message):
    with open('./tests/windows.junit.xml') as f:
        failure_message = f.read()

    partial_res = shorten_file_paths(failure_message)
    res = escape_failure_message(partial_res)
   
    assert res == """Error: expect(received).toBe(expected) // Object.is equality<br><br>Expected: 4<br>Received: 5<br>at Object.&amp;lt;anonymous&amp;gt;<br>(.../demo/calculator/calculator.test.ts:5:26)<br>at Promise.then.completed<br>(.../jest-circus/build/utils.js:298:28)<br>at new Promise (&amp;lt;anonymous&amp;gt;)<br>at callAsyncCircusFn<br>(.../jest-circus/build/utils.js:231:10)<br>at _callCircusTest<br>(.../jest-circus/build/run.js:316:40)<br>at processTicksAndRejections (node:internal/process/task_queues:95:5)<br>at _runTest<br>(.../jest-circus/build/run.js:252:3)<br>at _runTestsForDescribeBlock<br>(.../jest-circus/build/run.js:126:9)<br>at run<br>(.../jest-circus/build/run.js:71:3)<br>at runAndTransformResultsToJestFormat<br>(.../build/legacy-code-todo-rewrite/jestAdapterInit.js:122:21)<br>at jestAdapter<br>(.../build/legacy-code-todo-rewrite/jestAdapter.js:79:19)<br>at runTestInternal<br>(.../jest-runner/build/runTest.js:367:16)<br>at runTest<br>(.../jest-runner/build/runTest.js:444:34)"""


@pytest.mark.parametrize("escape_failure_message", [escape_failure_message_pystring, escape_failure_message_rust_string])
def test_escape_failure_message_happy_path(escape_failure_message):
    failure_message = "\"'<>&\r\n"
    res = escape_failure_message(failure_message)
    assert res == "&amp;quot;&amp;apos;&amp;lt;&amp;gt;&amp;<br>"

@pytest.mark.parametrize("escape_failure_message", [escape_failure_message_pystring, escape_failure_message_rust_string])
def test_escape_failure_message_slash_in_message(escape_failure_message):
    failure_message = "\\ \\n \n"
    res = escape_failure_message(failure_message)
    assert res == "\\ \\n <br>"

@pytest.mark.parametrize("shorten_file_paths", [shorten_file_paths_pystring, shorten_file_paths_rust_string])
def test_shorten_file_paths_short_path(shorten_file_paths):
    failure_message = "short/file/path.txt"
    res = shorten_file_paths(failure_message)
    assert res == failure_message

@pytest.mark.parametrize("shorten_file_paths", [shorten_file_paths_pystring, shorten_file_paths_rust_string])
def test_shorten_file_paths_long_path(shorten_file_paths):
    failure_message = "very/long/file/path/should/be/shortened.txt"
    res = shorten_file_paths(failure_message)
    assert res == ".../should/be/shortened.txt"

@pytest.mark.parametrize("shorten_file_paths", [shorten_file_paths_pystring, shorten_file_paths_rust_string])
def test_shorten_file_paths_long_path_leading_slash(shorten_file_paths):
    failure_message = "/very/long/file/path/should/be/shortened.txt"
    res = shorten_file_paths(failure_message)
    assert res == ".../should/be/shortened.txt"