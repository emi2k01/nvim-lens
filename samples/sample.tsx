import { User, BasicRole } from "../auth";

type Level = "low" | "medium" | "high";

interface ExtendedRole extends BasicRole {
  name: string;
  level: Level;
}

function ban_user(user: User) {
  try {
    if user.role.level === "high" {
      throw new Error("Can't ban user with a high role's level");
    }
    db.user.ban(user.id);
  } catch (e) {
    console.error(e);
    throw e;
  }
}

function DisplayUsers(props: { users: User[]}) {
  return (
    <div>
      <h1>Users</h1>
      <ul>
        {props.users.map((user) => (
          <li>{user.username}</li>
        ))}
      </ul>
    </div>
  )
}
