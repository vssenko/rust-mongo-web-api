import t from "node:test";
import assert from "node:assert";
import api from "./api.js";
import { registerUser } from "./helpers/user.js";

export function unauthorized({ method, url, data }) {
  return t.it(
    `${method} ${url} should prevent unauthorized access`,
    async () => {
      data = typeof data === "function" ? data() : data;

      const error = await api
        .getApi()
        .request({
          url,
          method,
          data,
        })
        .catch((e) => e);

      assert.equal(error.status, 401);
    },
  );
}

export async function unauthorizedForRole({ method, url, data, role, token }) {
  return t.it(
    `${method} ${url} should be not accessible for ${role}`,
    async () => {
      data = typeof data === "function" ? data() : data;
      token = typeof token === "function" ? token() : token;

      if (!token && !role)
        throw new Error("Either token or role should be provided.");

      if (!token) {
        token = (await registerUser({ role })).token;
      }

      const error = await api
        .getApi({ token })
        .request({
          url,
          method,
          data,
        })
        .catch((e) => e);

      assert.equal(error.status, 401);
    },
  );
}

export default {
  unauthorized,
  unauthorizedForRole,
};
