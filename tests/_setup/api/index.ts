import axios from 'axios';
import http from 'http';


let httpServer: http.Server;
let serverUrl: string;

export async function startApi() {
  throw new Error('Not implemented');
}

export interface GetApiParams {
  simplifyErrors?: boolean,
}

export function getApi({ simplifyErrors = true }: GetApiParams = {}) {
  const axiosInstance = axios.create();


  return axiosInstance;
}

export default {
  startApi,
  getApi
};