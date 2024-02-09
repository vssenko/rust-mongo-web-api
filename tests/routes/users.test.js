import test from "node:test";
import assert from "node:assert";
import context from "../_context/index.js";

test.describe("/users", () => {
  const email = "test@email.com";
  const password = "password123";
  let token;

  test.before(async (t) => await context.bootstrap());
  test.after(async (t) => await context.shutdown());

  test.it("post /login should fail for not existing user", async () => {
    await assert.rejects(async () =>
      context.api().post("/users/login", {
        email,
        password,
      }),
    );
  });

  test.it("post /register should create user", async () => {
    const result = await context.api().post("/users/register", {
      email,
      password,
    });

    assert.ok(result.data.token);

    const user = result.data.user;

    assert.equal(typeof user._id, "string");
    assert.equal(user.email, email);
    assert.equal(user.role, "User");
    assert.ok(!user.password);

    token = result.data.token;
  });

  test.it("get /me should return myself by token", async () => {
    const result = await context.api({ token }).get("/users/me");

    assert.equal(result.data.email, email);
  });

  test.it("post /login should login for existing user", async () => {
    const result = await context.api().post("/users/login", {
      email,
      password,
    });

    assert.ok(result.data.token);
    assert.equal(result.data.user.email, email);
  });

  context.test.unauthorizedForRole({
    method: "get",
    url: "/users",
    role: "User",
    token: () => token,
  });

  test.it("get /users should return all users for admin", async () => {
    let adminRegisterData = await context.user.registerUser({ role: "Admin" });

    const result = await context
      .api({ token: adminRegisterData.token })
      .get("/users");

    const users = result.data;

    assert.ok(users.find((u) => u._id === adminRegisterData.user._id));
    assert.ok(users.find((u) => u.email === email));
    assert.equal(users.length, 2);
  });
});
