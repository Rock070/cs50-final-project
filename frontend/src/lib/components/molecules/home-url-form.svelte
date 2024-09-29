<script lang="ts">
  import * as Form from "$lib/components/ui/form";
  import { Copy } from '$lib/components/ui/copy'
  import { Status } from '$lib/type'
  import { userStore } from '$lib/store/user'
  import type { HashUrlResponse } from '$lib/type/api'
  import { Input } from "$lib/components/ui/input";
  import StatusDialog from "./status-dialog.svelte";
  import { request } from "$lib/request";
  import { urlFormSchema, type UrlFormSchema } from "../../../routes/schema";
  import {
   type SuperValidated,
   type Infer,
   superForm,
  } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  
  export let data: SuperValidated<Infer<UrlFormSchema>>;
  
  let dialogOpen = false
  let status = Status.Success
  let shortenUrl = ''
  let errorMessage = ''

  const form = superForm(data, {
   validators: zodClient(urlFormSchema),
   SPA: true,
   clearOnSubmit: 'errors',
    onUpdate({ form }){
      if (!form.valid) return;
      request<HashUrlResponse>('/url/hash', {
        method: 'POST',
        token: $userStore.token,
        body: {
          url: form.data.url,
        },
      })
        .then(res => {
          shortenUrl = res.data.url
          status = Status.Success
        })
        .catch(err => {
          status = Status.Error
          errorMessage = err
        })
        .finally(() => {
          dialogOpen = true
        });
    },
  });
        

  const { form: formData, enhance } = form;
 </script>
  
 <form 
  method="POST"
  use:enhance
>
  <Form.Field {form} name="url">
   <Form.Control let:attrs>
    <Form.Label>Shorten your URL</Form.Label>
    <Input {...attrs} bind:value={$formData.url} placeholder="Enter the url here"/>
   </Form.Control>
   <Form.FieldErrors />
  </Form.Field>
  <div class="flex justify-end mt-3">
    <Form.Button>Shorten</Form.Button>
  </div>
 </form>

 <StatusDialog 
  bind:open={dialogOpen} 
  {status}
 >
 <div>
    {#if status === Status.Success}
    <div
      class="flex justify-between"
    >
      <span>your shorten URL is</span>
      <br />
      <a 
        href={shortenUrl} 
        target="_blank"
        rel="noopener noreferrer"
        class="text-blue-400"
      >
        {shortenUrl}
      </a>
      <Copy text={shortenUrl} />
    </div>
    {:else if status = Status.Error}
      <span>copy failed</span>
    {:else}
      <span />
    {/if}
 </div>
</StatusDialog>