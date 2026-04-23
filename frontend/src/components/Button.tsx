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
  "inline-flex items-center gap-2 font-sans font-medium tracking-tight whitespace-nowrap transition-all duration-150 disabled:opacity-40 disabled:cursor-not-allowed rounded-[6px] select-none";

const sizes: Record<Size, string> = {
  md: "px-4 py-2.5 text-[0.92rem]",
  sm: "px-3 py-1.5 text-[0.82rem]"
};

const variants: Record<Variant, string> = {
  primary:
    "bg-surface text-fg border border-line-strong hover:border-accent hover:text-accent",
  ghost:
    "bg-transparent text-fg-dim hover:text-fg border border-transparent hover:border-line",
  danger:
    "bg-transparent text-[color:var(--color-danger)] border border-[color:var(--color-danger)]/40 hover:bg-[color:var(--color-danger)]/10 hover:border-[color:var(--color-danger)]",
  outline:
    "bg-transparent text-fg-dim border border-line hover:border-accent hover:text-accent"
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
