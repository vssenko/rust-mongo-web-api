interface CreateUserParams {
  email?: string,
  password?: string
}

export async function createUser({ email, password }: CreateUserParams = {}) {
  throw new Error("Not implemented");
}

export default {
  createUser
}