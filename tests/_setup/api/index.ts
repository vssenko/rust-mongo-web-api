import axios from 'axios';
import cp from 'child_process';
//import { getFreePort } from '../utils'

const secondsToWaitAfterStart = 3;

let apiProcess: cp.ChildProcessWithoutNullStreams | null = null;
let serverUrl: string;

interface StartApiParams {
  mongourl: string
}

export async function startApi(params: StartApiParams) {
  const port = 3252//await getFreePort();

  apiProcess = cp.spawn(`cargo run`, [], {
    cwd: process.cwd(),
    shell: true,
    env: {
      MONGODB_URI: params.mongourl,
      PORT: port.toString(),
      THREAD_COUNT: '2'
    },
  });

  await new Promise(r => setTimeout(r, secondsToWaitAfterStart * 1000));

  serverUrl = `http://127.0.0.1:${port}`;

  return {
    serverUrl
  };
}

export async function stopApi() {
  if (!apiProcess) return;
  apiProcess.stdout.destroy()
  apiProcess.stdin.destroy()
  apiProcess.stderr.destroy()
  apiProcess.kill();
}

export interface GetApiParams {
  simplifyErrors?: boolean,
}

export function getApi({ simplifyErrors = true }: GetApiParams = {}) {
  const axiosInstance = axios.create({
    baseURL: serverUrl
  });


  return axiosInstance;
}

export default {
  startApi,
  stopApi,
  getApi
};