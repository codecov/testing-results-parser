from test_results_parser import escape_failure_message, shorten_file_paths


def test_escape_failure_message():
    with open('./tests/windows.junit.xml') as f:
        failure_message = f.read()
    res = escape_failure_message(failure_message)

    assert res == """Error: expect(received).toBe(expected) // Object.is equality<br><br>Expected: 4<br>Received: 5<br>at Object.&lt;anonymous&gt;<br>(/Users/user/dev/repo/demo/calculator/calculator.test.ts:5:26)<br>at Promise.then.completed<br>(/Users/user/dev/repo/node_modules/jest-circus/build/utils.js:298:28)<br>at new Promise (&lt;anonymous&gt;)<br>at callAsyncCircusFn<br>(/Users/user/dev/repo/node_modules/jest-circus/build/utils.js:231:10)<br>at _callCircusTest<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:316:40)<br>at processTicksAndRejections (node:internal/process/task_queues:95:5)<br>at _runTest<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:252:3)<br>at _runTestsForDescribeBlock<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:126:9)<br>at run<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:71:3)<br>at runAndTransformResultsToJestFormat<br>(/Users/user/dev/repo/node_modules/jest-circus/build/legacy-code-todo-rewrite/jestAdapterInit.js:122:21)<br>at jestAdapter<br>(/Users/user/dev/repo/node_modules/jest-circus/build/legacy-code-todo-rewrite/jestAdapter.js:79:19)<br>at runTestInternal<br>(/Users/user/dev/repo/node_modules/jest-runner/build/runTest.js:367:16)<br>at runTest<br>(/Users/user/dev/repo/node_modules/jest-runner/build/runTest.js:444:34)"""



def test_shorten_file_paths():
    with open('./tests/windows.junit.xml') as f:
        failure_message = f.read()

    res = shorten_file_paths(failure_message)

    assert res == """Error: expect(received).toBe(expected) // Object.is equality

Expected: 4
Received: 5
at Object.&lt;anonymous&gt;
(.../calculator.test.ts:5:26/calculator/demo)
at Promise.then.completed
(.../utils.js:298:28/build/jest-circus)
at new Promise (&lt;anonymous&gt;)
at callAsyncCircusFn
(.../utils.js:231:10/build/jest-circus)
at _callCircusTest
(.../run.js:316:40/build/jest-circus)
at processTicksAndRejections (node:internal/process/task_queues:95:5)
at _runTest
(.../run.js:252:3/build/jest-circus)
at _runTestsForDescribeBlock
(.../run.js:126:9/build/jest-circus)
at run
(.../run.js:71:3/build/jest-circus)
at runAndTransformResultsToJestFormat
(.../jestAdapterInit.js:122:21/legacy-code-todo-rewrite/build)
at jestAdapter
(.../jestAdapter.js:79:19/legacy-code-todo-rewrite/build)
at runTestInternal
(.../runTest.js:367:16/build/jest-runner)
at runTest
(.../runTest.js:444:34/build/jest-runner)"""

def test_both():
    with open('./tests/windows.junit.xml') as f:
        failure_message = f.read()

    partial_res = shorten_file_paths(failure_message)
    res = escape_failure_message(failure_message)
   
    assert res == """Error: expect(received).toBe(expected) // Object.is equality<br><br>Expected: 4<br>Received: 5<br>at Object.&lt;anonymous&gt;<br>(/Users/user/dev/repo/demo/calculator/calculator.test.ts:5:26)<br>at Promise.then.completed<br>(/Users/user/dev/repo/node_modules/jest-circus/build/utils.js:298:28)<br>at new Promise (&lt;anonymous&gt;)<br>at callAsyncCircusFn<br>(/Users/user/dev/repo/node_modules/jest-circus/build/utils.js:231:10)<br>at _callCircusTest<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:316:40)<br>at processTicksAndRejections (node:internal/process/task_queues:95:5)<br>at _runTest<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:252:3)<br>at _runTestsForDescribeBlock<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:126:9)<br>at run<br>(/Users/user/dev/repo/node_modules/jest-circus/build/run.js:71:3)<br>at runAndTransformResultsToJestFormat<br>(/Users/user/dev/repo/node_modules/jest-circus/build/legacy-code-todo-rewrite/jestAdapterInit.js:122:21)<br>at jestAdapter<br>(/Users/user/dev/repo/node_modules/jest-circus/build/legacy-code-todo-rewrite/jestAdapter.js:79:19)<br>at runTestInternal<br>(/Users/user/dev/repo/node_modules/jest-runner/build/runTest.js:367:16)<br>at runTest<br>(/Users/user/dev/repo/node_modules/jest-runner/build/runTest.js:444:34)"""