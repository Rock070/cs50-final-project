<script lang="ts">
  import { page } from '$app/stores';
  import type { PageData } from './$types.js';
  import RegisterForm from '$lib/components/molecules/register-form.svelte';

  export let data: PageData;
</script>

<h2 class="text-2xl font-bold text-center mb-4">Register</h2>
<div
  class="
    relative -top-[5%]
    w-[90%] md:w-2/3
    h-full
    mx-auto
    flex items-center justify-center
    *:shrink-0 
    *:grow
  "
>
  <RegisterForm 
    data={data.form} 
  />
</div>

