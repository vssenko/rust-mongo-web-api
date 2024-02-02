import test, { TestFn, ExecutionContext as EC } from 'ava';
import { getApi, GetApiParams } from '../_setup/api';
import testHelpers from '../_setup/helpers';

import type { Server } from 'http';
import type { MongoMemoryServer } from 'mongodb-memory-server';
import type { AxiosInstance } from 'axios';

export type GetApiFunction = (params?: GetApiParams) => AxiosInstance;

export interface TestContext {
  mongodb: MongoMemoryServer,
  httpServer: Server,
  serverUrl: string,
  api: typeof getApi,
  helpers: typeof testHelpers,
  [x: string | number | symbol]: any;
}

export type ExecutionContext = EC<TestContext>;

export default test as TestFn<TestContext>;

