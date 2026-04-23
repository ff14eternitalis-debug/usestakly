import type { CurrentUser } from "../../../lib/types";

type AccountIdentityCardProps = {
  user: CurrentUser | null;
};

export function AccountIdentityCard({ user }: AccountIdentityCardProps) {
  return (
    <div className="surface grid gap-2 p-4">
      <span className="kicker">{user?.username ? `@${user.username}` : "@"}</span>
      <p className="text-[0.9rem] text-fg-dim">{user?.email}</p>
    </div>
  );
}
