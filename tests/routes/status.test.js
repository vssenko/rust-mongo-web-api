import test from "node:test";
import assert from "node:assert";
import context from "../_context/index.js";

test.describe("/status", () => {
  test.before(async (t) => await context.bootstrap());
  test.after(async (t) => await context.shutdown());

  test.it("should return status ok", async () => {
    const result = await context.api().get("/status");
    assert.deepEqual(result.data, {
      status: "Ok",
    });
  });
});
