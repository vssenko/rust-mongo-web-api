import mongo from "../mongo.js";
import { v4 as uuid } from "uuid";
import { getApi } from "../api.js";

const defaultPassword = "1qaz!QAZ";

export async function registerUser({ email, password, role } = {}) {
  email = email || `test-${uuid()}@test.com`;
  password = password || defaultPassword;
  const result = await getApi().post("/users/register", {
    email,
    password,
  });

  const registerData = result.data;

  if (role) {
    const db = await mongo.getDatabase();
    const collection = db.collection("users");
    await collection.find({});
    await collection.findOneAndUpdate(
      {
        _id: registerData.user._id,
      },
      {
        $set: {
          role,
        },
      },
    );

    registerData.user.role = role;
  }

  return registerData;
}

export default {
  registerUser,
};
