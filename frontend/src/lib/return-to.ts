export function currentReturnTo(): string {
  return `${window.location.pathname}${window.location.search}${window.location.hash}`;
}

export function loginSearch() {
  return { returnTo: currentReturnTo() };
}

export function authPath(path: string, returnTo: string): string {
  const params = new URLSearchParams();
  if (isSafeReturnTo(returnTo)) {
    params.set("return_to", returnTo);
  }
  const query = params.toString();
  return query ? `${path}?${query}` : path;
}

export function loginReturnTo(): string {
  const value = new URLSearchParams(window.location.search).get("returnTo");
  return isSafeReturnTo(value) ? value : "/";
}

function isSafeReturnTo(value: string | null): value is string {
  return Boolean(
    value &&
      value.startsWith("/") &&
      !value.startsWith("//") &&
      !value.includes("\\")
  );
}
