<script lang="ts">
  import Icon from '@iconify/svelte';
  import { Status } from '$lib/type'
  import * as Dialog from "$lib/components/ui/dialog";
  export let status: Status = Status.Info;
  export let open = false;

 </script>
  
 <Dialog.Root
  bind:open={open}
 >
  <Dialog.Content>
   <Dialog.Header>
    <div
      class="mx-auto"
    >
      {#if status === Status.Info}
        <Icon 
          icon="radix-icons:info-circled"
          class="
            text-blue-500
            text-5xl
          "
        />
      {:else if status === Status.Success}
        <Icon 
          icon="radix-icons:check-circled"
          class="
            text-green-500
            text-5xl
          "
        />
      {:else if status === Status.Error}
        <Icon 
          icon="radix-icons:cross-circled"
          class="
          text-red-500
            text-5xl
          "
        />
      {:else}
        <div />
      {/if}
    </div>
    <!-- <Dialog.Title> 
      {#if status === Status.Info}
        <p>Info</p>
      {:else if status === Status.Success}
        <p>Success</p>
      {:else if status === Status.Error}
        <p>Error</p>
      {:else}
        <p></p>
      {/if}
    </Dialog.Title> -->
    <Dialog.Description>
     <slot />
    </Dialog.Description>
   </Dialog.Header>
  </Dialog.Content>
 </Dialog.Root>