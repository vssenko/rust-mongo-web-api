import test from 'node:test'
import assert from 'node:assert'
import context from '../_context/index.js'

test.describe('/users', () => {
  let email;
  let password;
  let token;

  test.before(async t => await context.bootstrap());
  test.after(async t => await context.shutdown());

  test.before(() => {
    email = 'test@email.com';
    password = 'password123';
  })

  test.it('/register should create user', async () => {
    const result = await context.api().post('/users/register', {
      email,
      password
    });

    assert.ok(result.data.token);

    const user = result.data.user;

    assert.equal(typeof user._id, 'string');
    assert.equal(user.email, email);
    assert.equal(user.role, 'User');
    assert.ok(!user.password);

    token = result.data.token;
  });

  test.it('/me should return myself by token', async () => {
    const result = await context.api({ token }).get('/users/me');

    assert.equal(result.data.email, email);
  });
});