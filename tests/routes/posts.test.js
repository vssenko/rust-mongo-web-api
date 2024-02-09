import test from "node:test";
import assert from "node:assert";
import context from "../_context/index.js";

test.describe("/posts", () => {
  let registerData;

  test.before(async (t) => await context.bootstrap());
  test.after(async (t) => await context.shutdown());

  test.before(async () => {
    registerData = await context.user.registerUser();
  });

  test.it("get /posts should return no posts", async () => {
    const result = await context.api().get("/posts");
    assert.deepEqual(result.data, []);
  });

  context.test.unauthorized({
    url: "/posts",
    method: "post",
    data: { title: "Some title", content: "Some content" },
  });

  test.it("post /posts should create post for authorized user", async () => {
    const result = await context
      .api({ token: registerData.token })
      .post("/posts", {
        title: "Some title",
        content: "Some content",
      });

    assert.ok(result.data._id);
    assert.equal(result.data.title, "Some title");
    assert.equal(result.data.user_id, registerData.user._id);
  });
});
