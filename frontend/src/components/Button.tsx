import type {
  AnchorHTMLAttributes,
  ButtonHTMLAttributes,
  ReactNode
} from "react";

type Variant = "primary" | "ghost" | "danger" | "outline";
type Size = "md" | "sm";

type CommonProps = {
  variant?: Variant;
  size?: Size;
  iconAfter?: ReactNode;
  iconBefore?: ReactNode;
};

const base =
  "inline-flex items-center gap-2 font-sans font-semibold tracking-tight whitespace-nowrap transition-all duration-150 disabled:opacity-40 disabled:cursor-not-allowed";

const sizes: Record<Size, string> = {
  md: "px-5 py-3 text-[0.92rem] rounded-sm",
  sm: "px-3 py-1.5 text-[0.82rem] rounded-sm"
};

const variants: Record<Variant, string> = {
  primary:
    "bg-ink text-paper-soft border border-ink hover:bg-ink-dim hover:border-ink-dim",
  ghost:
    "bg-transparent text-ink-dim hover:text-ink border border-transparent hover:border-line",
  danger:
    "bg-ember text-paper-soft border border-ember hover:bg-[color:#7c1f0f] hover:border-[color:#7c1f0f]",
  outline:
    "bg-transparent text-ink border border-ink hover:bg-ink hover:text-paper-soft"
};

export function buttonClass(
  variant: Variant = "primary",
  size: Size = "md",
  extra = ""
): string {
  return `${base} ${sizes[size]} ${variants[variant]} ${extra}`.trim();
}

export function Button({
  variant = "primary",
  size = "md",
  iconAfter,
  iconBefore,
  className = "",
  children,
  ...rest
}: CommonProps & ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      className={`${base} ${sizes[size]} ${variants[variant]} ${className}`}
      {...rest}
    >
      {iconBefore}
      {children}
      {iconAfter}
    </button>
  );
}

export function LinkButton({
  variant = "primary",
  size = "md",
  iconAfter,
  iconBefore,
  className = "",
  children,
  ...rest
}: CommonProps & AnchorHTMLAttributes<HTMLAnchorElement>) {
  return (
    <a
      className={`${base} ${sizes[size]} ${variants[variant]} ${className}`}
      {...rest}
    >
      {iconBefore}
      {children}
      {iconAfter}
    </a>
  );
}
