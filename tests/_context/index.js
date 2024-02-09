import { getApi } from "./api.js";
import { bootstrap, shutdown } from "./bootstrap.js";

export * from "./helpers/user.js";

export default {
  api: getApi,
  bootstrap,
  shutdown,
  user: {
    createUser,
  },
};
