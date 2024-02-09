import { getApi } from './api.js';
import { bootstrap, shutdown} from './bootstrap.js'

export async function createUser({ email, password } = {}) {
  throw new Error("Not implemented");
}

export default {
  api: getApi,
  bootstrap,
  shutdown,
  user: {
    createUser
  }
}