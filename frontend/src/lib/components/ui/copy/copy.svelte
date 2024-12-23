<script>
  import Icon from '@iconify/svelte';
  import * as Tooltip from "$lib/components/ui/tooltip";
  import { copyText } from '$lib/utils'

  export let text = '';
  let copied = false;
  let error = false;

  const onClick = async () => {
    await copyText(text, {
      onSuccess() {
        copied = true
        error = false
        setTimeout(() => copied = false, 3e3); // Reset after 5 seconds
      },
      onError(err) {
        error = true
        console.log(JSON.stringify(err))
      }
    })

  }
</script>


<Tooltip.Root
  openDelay={200}
>
  <Tooltip.Trigger>
    {#if !!copied}
      copied
    {:else if !!error}
      <span>copied fail</span>
    {:else}
      <button 
        type="button"
        on:click={async () => await onClick()}
      >
        <Icon 
        icon="radix-icons:copy"
        class="
          black-blue-500
          text-xl
        "
        />
      </button>
    {/if}
  </Tooltip.Trigger>
  {#if !copied && !error}
    <Tooltip.Content>
      copy
    </Tooltip.Content>
  {/if}
</Tooltip.Root>