<script lang="ts">
  import * as Form from "$lib/components/ui/form";
  import { Copy } from '$lib/components/ui/copy'
  import { Status } from '$lib/type'
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
      console.log(form);
      if (!form.valid) return;
      request<HashUrlResponse>('/hash-url', {
        method: 'POST',
        body: {
          url: form.data.url,
        },
      })
        .then(res => {
          console.log("🚀 ~ onUpdate ~ res:", res)
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
    <Form.Label>url</Form.Label>
    <Input {...attrs} bind:value={$formData.url} placeholder="Please enter a valid URL to shorten."/>
   </Form.Control>
   <Form.FieldErrors />
  </Form.Field>
  <Form.Button>Submit</Form.Button>
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
      <span>your shorten url is: {shortenUrl}</span>
      <Copy text={shortenUrl} />
    </div>
    {:else if status = Status.Error}
      <span>copy failed</span>
    {:else}
      <span />
    {/if}
 </div>
</StatusDialog>