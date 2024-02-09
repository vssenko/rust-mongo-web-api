import _ from 'lodash';
import axios from 'axios';
import { spawn, shutdownProcess } from './spawn.js';

const secondsToWaitAfterStart = 3;

let apiProcess = null;
let serverUrl = null;

class AxiosSimpleError extends Error {
  constructor(axiosError) {
    super(axiosError.message);
    this._isAxiosError = true;
    this.config = _.pick(axiosError.config, ['headers', 'method', 'baseURL', 'url', 'data']);
    this.response = _.pick(axiosError.response, ['status', 'statusText', 'headers', 'data']);
    this.status = axiosError.response?.status;

    const responseBody = axiosError?.response?.data;
    if (responseBody && typeof responseBody === 'object') {
      Object.keys(responseBody).forEach((k) => {
        this[k] = responseBody[k];
      });
    }
  }
}


export async function startApi({ mongourl, port }) {
  apiProcess = await spawn({
    command: `cargo run`,
    args: [],
    options: {
      cwd: process.cwd(),
      shell: true,
      env: {
        MONGODB_URI: mongourl,
        PORT: port,
        THREAD_COUNT: '2'
      },
    },
    waitForSeconds: 2
  });
  
  await new Promise(r => setTimeout(r, secondsToWaitAfterStart * 1000));

  serverUrl = `http://127.0.0.1:${port}`;

  console.log(`api: started with url "${serverUrl}"`)

  return {
    serverUrl
  };
}

export async function stopApi() {
  if (!apiProcess) return;
  shutdownProcess(apiProcess);
}

export function getApi({ token, simplifyErrors = true } = {}) {
  const headers = {};

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const axiosInstance = axios.create({
    baseURL: serverUrl,
    headers
  });

  if (simplifyErrors) {
    axiosInstance.interceptors.response.use(
      (response) => response,
      (error) => {
        return Promise.reject(new AxiosSimpleError(error));
      }
    );
  }


  return axiosInstance;
}

export default {
  startApi,
  stopApi,
  getApi
};