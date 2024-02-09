import cp from "child_process";

/**
 * @param {Object} obj
 * @param {string} obj.command
 * @param {string[]} obj.args
 * @param {cp.SpawnOptionsWithoutStdio} obj.options,
 * @param {string} obj.waitForOutput
 * @param {number} obj.waitForSeconds
 * @returns {cp.ChildProcessWithoutNullStreams}
 */
export async function spawn({
  command,
  args,
  options,
  waitForSeconds,
  waitForOutput,
  timeoutSeconds = 20,
}) {
  const childProcess = cp.spawn(command, args, options);

  let logs = "";
  childProcess.stdout.on("data", (data) => {
    const strdata = data.toString();
    logs += strdata;
    console.log(strdata);
  });

  childProcess.stderr.on("data", (data) => console.error(data.toString()));

  const start = Date.now();

  await new Promise((resolve) => {
    if (waitForSeconds) {
      return setTimeout(resolve, waitForSeconds * 1000);
    }

    if (waitForOutput) {
      const intervalHandler = setInterval(() => {
        if (logs.includes(waitForOutput)) {
          clearInterval(intervalHandler);
          return resolve();
        }

        if (Date.now() > start + timeoutSeconds * 1000) {
          clearInterval(intervalHandler);
          return reject(new Error("spawn: Timeout."));
        }
      }, 100);
      return;
    }

    return resolve();
  });

  return childProcess;
}

export function shutdownProcess(childProcess) {
  if (!childProcess) return;
  childProcess.stdout?.destroy();
  childProcess.stdin?.destroy();
  childProcess.stderr?.destroy();
  childProcess.kill();
}
