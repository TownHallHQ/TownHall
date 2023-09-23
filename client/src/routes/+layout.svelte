<script lang="ts">
  import { cacheExchange, fetchExchange } from "@urql/core";
  import { authExchange } from "@urql/exchange-auth";
  import { createClient, setContextClient } from "@urql/svelte";
  import { NotificationList, Position } from "@whizzes/svelte-notifications";

  import { page } from "$app/stores";
  import { browser } from "$app/environment";
  import Notification from "$lib/components/Notification/Notification.svelte";
  import ui from "$lib/stores/ui";

  import "@fontsource/inter";
  import "../app.css";

  import { pwaInfo } from "virtual:pwa-info";
  import { onMount } from "svelte";

  const initializeAuthState = () => {
    const token = $page.data?.accessToken;

    return {
      token,
    };
  };

  const client = createClient({
    url: import.meta.env.VITE_GRAPHQL_URL,
    exchanges: [
      cacheExchange,
      authExchange(async (utils) => {
        // https://formidable.com/open-source/urql/docs/advanced/authentication/
        const { token } = initializeAuthState();

        return {
          addAuthToOperation(operation) {
            if (!token) {
              return operation;
            }

            return utils.appendHeaders(operation, {
              Authorization: `JWT ${token}`,
            });
          },
          async refreshAuth() {
            window.location.href = "/logout";
          },
          willAuthError() {
            return false;
          },
          didAuthError() {
            return false;
          },
        };
      }),
      fetchExchange,
    ],
  });

  if (browser) {
    ui.syncPreferredScheme();
  }

  setContextClient(client);

  onMount(async () => {
    if (pwaInfo) {
      // @ts-ignore
      const { registerSW } = await import("virtual:pwa-register");
      registerSW({
        immediate: true,
        onRegistered(r: any) {
          console.log(`SW Registered: ${r}`);
        },
        onRegisterError(error: any) {
          console.log("SW registration error", error);
        },
      });
    }
  });

  $: webManifest = pwaInfo ? pwaInfo.webManifest.linkTag : "";
</script>

<svelte:head>
  {@html webManifest}
</svelte:head>

<div
  class="font-inter bg-slate-50 text-black dark:bg-neutral-800 dark:text-white"
>
  <slot />
</div>

<NotificationList position={Position.TopRight} let:notification>
  <Notification {notification} />
</NotificationList>

{#await import("$lib/components/ReloadPrompt.svelte") then { default: ReloadPrompt }}
  <ReloadPrompt />
{/await}
