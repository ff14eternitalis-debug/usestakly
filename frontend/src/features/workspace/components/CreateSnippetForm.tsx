import { useEffect, useMemo, useState } from "react";
import type { FormEvent } from "react";

import type { CopyBlock, LibraryRecord } from "../../../lib/app-types";

type CreateSnippetFormProps = {
  copy: CopyBlock;
  libraries: LibraryRecord[];
  onCreate: (input: {
    libraryId: string;
    slug: string;
    name: string;
    domain: string;
    kind: string;
    category: string;
    language: string;
    framework?: string;
    tags: string[];
    version: string;
    code: string;
  }) => Promise<void>;
};

export function CreateSnippetForm({
  copy,
  libraries,
  onCreate
}: CreateSnippetFormProps) {
  const defaultLibraryId = useMemo(
    () => libraries.find((library) => library.isDefault)?.id ?? libraries[0]?.id ?? "",
    [libraries]
  );

  const [libraryId, setLibraryId] = useState(defaultLibraryId);
  const [name, setName] = useState("");
  const [slug, setSlug] = useState("snippet-name");
  const [domain, setDomain] = useState("frontend");
  const [kind, setKind] = useState("component");
  const [category, setCategory] = useState("ui");
  const [language, setLanguage] = useState("typescript");
  const [framework, setFramework] = useState("react");
  const [version, setVersion] = useState("0.1.0");
  const [tags, setTags] = useState("react,ui");
  const [code, setCode] = useState("export function Example() {\n  return <button>Click</button>;\n}");
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (!libraryId && defaultLibraryId) {
      setLibraryId(defaultLibraryId);
    }
  }, [defaultLibraryId, libraryId]);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitting(true);
    try {
      await onCreate({
        libraryId,
        slug,
        name,
        domain,
        kind,
        category,
        language,
        framework: framework.trim() || undefined,
        version,
        code,
        tags: tags
          .split(",")
          .map((tag) => tag.trim())
          .filter(Boolean)
      });
      setName("");
      setSlug("snippet-name");
      setCode("export function Example() {\n  return <button>Click</button>;\n}");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form className="workspace-form" onSubmit={(event) => void handleSubmit(event)}>
      <div className="workspace-form-grid workspace-form-grid-3">
        <label className="workspace-field">
          <span>{copy.createSnippetLibrary}</span>
          <select
            value={libraryId}
            onChange={(event) => setLibraryId(event.target.value)}
            required
          >
            {libraries.map((library) => (
              <option key={library.id} value={library.id}>
                {library.name}
              </option>
            ))}
          </select>
        </label>
        <label className="workspace-field">
          <span>{copy.createSnippetName}</span>
          <input value={name} onChange={(event) => setName(event.target.value)} required />
        </label>
        <label className="workspace-field">
          <span>{copy.createSnippetSlug}</span>
          <input value={slug} onChange={(event) => setSlug(event.target.value)} required />
        </label>
      </div>

      <div className="workspace-form-grid workspace-form-grid-4">
        <label className="workspace-field">
          <span>{copy.createSnippetDomain}</span>
          <select value={domain} onChange={(event) => setDomain(event.target.value)}>
            <option value="frontend">frontend</option>
            <option value="backend">backend</option>
            <option value="data">data</option>
            <option value="devops">devops</option>
            <option value="shared">shared</option>
          </select>
        </label>
        <label className="workspace-field">
          <span>{copy.createSnippetKind}</span>
          <input value={kind} onChange={(event) => setKind(event.target.value)} required />
        </label>
        <label className="workspace-field">
          <span>{copy.createSnippetCategory}</span>
          <input
            value={category}
            onChange={(event) => setCategory(event.target.value)}
            required
          />
        </label>
        <label className="workspace-field">
          <span>{copy.createSnippetLanguage}</span>
          <input
            value={language}
            onChange={(event) => setLanguage(event.target.value)}
            required
          />
        </label>
      </div>

      <div className="workspace-form-grid workspace-form-grid-3">
        <label className="workspace-field">
          <span>{copy.createSnippetFramework}</span>
          <input value={framework} onChange={(event) => setFramework(event.target.value)} />
        </label>
        <label className="workspace-field">
          <span>{copy.createSnippetVersion}</span>
          <input value={version} onChange={(event) => setVersion(event.target.value)} required />
        </label>
        <label className="workspace-field">
          <span>{copy.createSnippetTags}</span>
          <input value={tags} onChange={(event) => setTags(event.target.value)} />
        </label>
      </div>

      <label className="workspace-field">
        <span>{copy.createSnippetCode}</span>
        <textarea rows={8} value={code} onChange={(event) => setCode(event.target.value)} required />
      </label>

      <button className="auth-primary-button workspace-submit" disabled={submitting} type="submit">
        {copy.createSnippetSubmit}
      </button>
    </form>
  );
}
