import { useState } from "react";
import type { FormEvent } from "react";

import type { CopyBlock } from "../../../lib/app-types";

type CreateLibraryFormProps = {
  copy: CopyBlock;
  onCreate: (input: {
    name: string;
    slug: string;
    description?: string;
    visibility: "private" | "public";
  }) => Promise<void>;
};

export function CreateLibraryForm({ copy, onCreate }: CreateLibraryFormProps) {
  const [name, setName] = useState("");
  const [slug, setSlug] = useState("@me/core-library");
  const [description, setDescription] = useState("");
  const [visibility, setVisibility] = useState<"private" | "public">("private");
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    try {
      await onCreate({
        name,
        slug,
        description: description.trim() || undefined,
        visibility
      });
      setName("");
      setSlug("@me/core-library");
      setDescription("");
      setVisibility("private");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form className="workspace-form" onSubmit={(event) => void handleSubmit(event)}>
      <div className="workspace-form-grid">
        <label className="workspace-field">
          <span>{copy.createLibraryName}</span>
          <input value={name} onChange={(event) => setName(event.target.value)} required />
        </label>
        <label className="workspace-field">
          <span>{copy.createLibrarySlug}</span>
          <input value={slug} onChange={(event) => setSlug(event.target.value)} required />
        </label>
      </div>
      <label className="workspace-field">
        <span>{copy.createLibraryDescription}</span>
        <textarea
          rows={3}
          value={description}
          onChange={(event) => setDescription(event.target.value)}
        />
      </label>
      <label className="workspace-field">
        <span>{copy.createLibraryVisibility}</span>
        <select value={visibility} onChange={(event) => setVisibility(event.target.value as "private" | "public")}>
          <option value="private">{copy.visibilityPrivate}</option>
          <option value="public">{copy.visibilityPublic}</option>
        </select>
      </label>
      <button className="auth-primary-button workspace-submit" disabled={submitting} type="submit">
        {copy.createLibrarySubmit}
      </button>
    </form>
  );
}
