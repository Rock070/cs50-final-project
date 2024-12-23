<script lang="ts">
  import { page } from '$app/stores';
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";
  import UrlsTable from "$lib/components/molecules/urls-table.svelte";
  import { userStore, clearUserStore } from '$lib/store/user';
  import { Button } from '$lib/components/ui/button';
  import { COOKIE_KEY } from '$lib/constant';
  import Cookie from 'js-cookie';
  import { goto } from '$app/navigation';

  import type { UserUrl } from '$lib/type/api';

  const logout = () => {
    Cookie.remove(COOKIE_KEY.TOKEN);
    clearUserStore();
    goto('/login')
  }

  export let data: { urls:  UserUrl[] } = { urls: [] };

  $: urls = data.urls;
</script>
<div class="flex flex-col items-center justify-center min-h-[70dvh]">
  <div
    class="w-full"
  >
    <p
      class="mb-4 text-right"
    >
      Hello, 
      <strong>{$userStore.username.slice(0, 1).toUpperCase() + $userStore.username.slice(1)}!</strong>
    </p>
    <div>
      <h3 class="text-lg font-medium">Profile</h3>
      <p class="text-muted-foreground text-sm">This is information about you.</p>
    </div>

    <Separator 
      class="mt-2 mb-6"
    />

    <div
      class="flex flex-col gap-y-5"
    >
      <div
        class="row-block"
      >
        <Label>Username</Label>
        <Input placeholder="username" value={$userStore.username} disabled />
      </div>

      <div
        class="row-block"
      >
        <Label>Email</Label>
        <Input placeholder="email" value={$userStore.email} disabled />
      </div>

      <div
        class="row-block"
      >
        <div>
          <Label>URLs</Label>
          <p class="text-muted-foreground text-sm">A list of your short urls.</p>
        </div>
        <UrlsTable urls={data.urls} />
      </div>
    </div>

  </div>
  <Button
    on:click={logout}
    class="mt-20"
  >
    Logout
  </Button>
</div>

<style>
.row-block {
  @apply flex flex-col gap-y-2;
}
</style>