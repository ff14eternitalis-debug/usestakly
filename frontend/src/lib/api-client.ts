const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? "http://localhost:4000";

export class ApiError extends Error {
  status: number;
  constructor(status: number, message: string) {
    super(message);
    this.status = status;
    this.name = "ApiError";
  }
}

async function request<T>(
  path: string,
  init: RequestInit & { parseJson?: boolean } = {}
): Promise<T> {
  const { parseJson = true, ...rest } = init;
  const response = await fetch(`${API_BASE_URL}${path}`, {
    credentials: "include",
    ...rest,
    headers: {
      ...(rest.body instanceof FormData
        ? {}
        : rest.body
          ? { "Content-Type": "application/json" }
          : {}),
      ...rest.headers
    }
  });

  if (!response.ok) {
    let detail = "";
    try {
      const contentType = response.headers.get("content-type") ?? "";
      if (contentType.includes("application/json")) {
        const body = (await response.json()) as { error?: string };
        detail = body.error ?? "";
      } else {
        detail = (await response.text()).slice(0, 200);
      }
    } catch {
      /* ignore */
    }
    throw new ApiError(
      response.status,
      detail || `Request failed with ${response.status}`
    );
  }

  if (!parseJson || response.status === 204) {
    return undefined as T;
  }
  return (await response.json()) as T;
}

export function apiGet<T>(path: string, signal?: AbortSignal): Promise<T> {
  return request<T>(path, { method: "GET", signal });
}

export function apiGetWithInit<T>(
  path: string,
  init: RequestInit & { parseJson?: boolean } = {}
): Promise<T> {
  return request<T>(path, { method: "GET", ...init });
}

export function apiPost<T>(
  path: string,
  body?: unknown,
  signal?: AbortSignal
): Promise<T> {
  return request<T>(path, {
    method: "POST",
    body: body === undefined ? undefined : JSON.stringify(body),
    signal
  });
}

export function apiPostWithInit<T>(
  path: string,
  body?: unknown,
  init: RequestInit & { parseJson?: boolean } = {}
): Promise<T> {
  return request<T>(path, {
    method: "POST",
    body: body === undefined ? undefined : JSON.stringify(body),
    ...init
  });
}

export function apiPatch<T>(
  path: string,
  body: unknown,
  signal?: AbortSignal
): Promise<T> {
  return request<T>(path, {
    method: "PATCH",
    body: JSON.stringify(body),
    signal
  });
}

export function apiDelete<T>(path: string, signal?: AbortSignal): Promise<T> {
  return request<T>(path, {
    method: "DELETE",
    parseJson: false,
    signal
  });
}

export function authUrl(path: string): string {
  return `${API_BASE_URL}${path}`;
}
