<script lang="ts">
  import Modal from "$lib/components/ui/modal.svelte";
  import DesktopIcon from "$lib/icons/desktop-icon.svelte";
  import UrlIcon from "$lib/icons/url-icon.svelte";
  import { onDestroy } from "svelte";
  import ActionCard from "./action-card.svelte";
  import FileForm from "./file-form.svelte";
  import Preview from "./preview.svelte";
  import UrlForm from "./url-form.svelte";
  import { browser } from "$app/environment";
  export let show = false;
  export let currentCard: "url" | "local" | "preview" | undefined = undefined;
  let preview: {
    type: "url" | "local";
    url: string;
  };
  let selectedFile: File | null;

  function handleFileChange(ev: Event) {
    const files = (ev.target as HTMLInputElement).files;
    if (!files?.length) return;
    const file = files[0];
    preview = {
      ...preview,
      url: URL.createObjectURL(file),
      type: "local",
    };
    selectedFile = file;
    currentCard = "preview";
  }

  function handleFileUpload() {
    console.log(selectedFile);
  }

  function destroyLocalBlob() {
    if (preview?.type === "local") {
      URL.revokeObjectURL(preview.type);
    }
    selectedFile = null;
  }

  if (browser) {
    onDestroy(() => {
      destroyLocalBlob();
    });
  }
</script>

<Modal bind:show>
  {#if currentCard !== undefined}
    <div class="mb-4">
      <button
        on:click={() => (currentCard = undefined)}
        class="dark:bg-neutral-600 px-4 py-2 rounded-full dark:hover:bg-neutral-500"
      >
        Back
      </button>
    </div>
  {/if}
  {#if currentCard === "url"}
    <UrlForm
      on:urlLoad={(ev) => {
        destroyLocalBlob();
        preview = {
          ...preview,
          url: ev.detail.url,
          type: "url",
        };
        currentCard = "preview";
      }}
    />
  {:else if currentCard === "local"}
    <FileForm on:change={handleFileChange} />
  {:else if currentCard === "preview"}
    <Preview on:fileUpload={handleFileUpload} url={preview?.url} />
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <ActionCard
        text="Upload from URL"
        cardType="url"
        on:uploadFrom={(ev) => (currentCard = ev.detail.cardType)}
      >
        <div slot="icon">
          <UrlIcon />
        </div>
      </ActionCard>
      <ActionCard
        cardType="local"
        on:uploadFrom={(ev) => (currentCard = ev.detail.cardType)}
        text="Upload from Local File"
      >
        <div slot="icon">
          <DesktopIcon />
        </div>
      </ActionCard>
    </div>
  {/if}
</Modal>
