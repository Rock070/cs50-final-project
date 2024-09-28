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

  const logout = () => {
    Cookie.remove(COOKIE_KEY.TOKEN);
    clearUserStore();
  }

  const urls = [ 
    {
      url: 'https://www.google.com',
      shortUrl: 'https://short.url/google',
    },
    {
      url: 'https://www.facebook.com',
      shortUrl: 'https://short.url/facebook',
    },
    {
      url: 'https://www.twitter.com',
      shortUrl: 'https://short.url/twitter',
    }
  ]
</script>
<div class="flex flex-col items-center justify-center min-h-[70dvh]">
  {#if $userStore.token}
  <div
    class="w-full"
  >
    <p>Hello, <strong>{$userStore.username.slice(0, 1).toUpperCase() + $userStore.username.slice(1)}!</strong></p>
    <div>
      <h3 class="text-lg font-medium">Profile</h3>
      <p class="text-muted-foreground text-sm">This is information about you.</p>
    </div>

    <Separator 
      class="mt-2 mb-6"
    />

    <div
      class="flex flex-col gap-3"
    >
      <div>
        <Label class="mb-3">Username</Label>
        <Input placeholder="username" value={$userStore.username} disabled />
      </div>

      <div>
        <Label class="mb-3">Email</Label>
        <Input placeholder="email" value={$userStore.email} disabled />
      </div>

      <div>
        <Label class="mb-3">URLs</Label>
        {#each urls as url}
          <Input placeholder="email" value={url} disabled />
        {/each}
      </div>
      <UrlsTable urls={urls} />


    </div>

  </div>
  <Button
    on:click={logout}
    class="mt-20"
  >
    Logout
  </Button>
  {:else}
  <a
    href="/login"
    rel="noopener"
    class:active-link={$page.url.pathname === '/login'}
  >
    Login
  </a>
  <a
    href="/register"
    rel="noopener"
    class:active-link={$page.url.pathname === '/register'}
  >
    Register
  </a>
  {/if}
</div>
