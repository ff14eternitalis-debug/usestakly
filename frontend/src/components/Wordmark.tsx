type Props = {
  scale?: "sm" | "md" | "lg";
  muted?: boolean;
};

export function Wordmark({ scale = "md", muted = false }: Props) {
  const heightClass =
    scale === "sm"
      ? "h-[1.05rem]"
      : scale === "lg"
        ? "h-[1.72rem]"
        : "h-[1.32rem]";
  const fade = muted ? "opacity-55" : "";
  return (
    <span className={`inline-flex items-center ${heightClass} ${fade}`}>
      <img
        src="/usestackly-logo-white-lime.svg"
        alt="UseStakly"
        className="h-full w-auto max-w-[min(200px,42vw)] object-contain object-left"
        draggable={false}
      />
    </span>
  );
}
