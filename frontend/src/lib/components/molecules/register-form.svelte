<script lang="ts">
  import * as Form from "$lib/components/ui/form";
  import { Button } from "$lib/components/ui/button";
  import { Status } from '$lib/type'
  import { goto } from '$app/navigation';
  import { Input } from "$lib/components/ui/input";
  import StatusDialog from "./status-dialog.svelte";
  import { request } from "$lib/request";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { registerFormSchema, type RegisterFormSchema } from "../../../routes/schema";
  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";
  import type { RegisterResponse } from '$lib/type/api'
  
  export let data: SuperValidated<Infer<RegisterFormSchema>>;
  
  let dialogOpen = false
  let status = Status.Success
  let errorMessage = ''
  let username = ''
  const form = superForm(data, {
   validators: zodClient(registerFormSchema),
   SPA: true,
   clearOnSubmit: 'errors',
    onUpdate({ form }){
      if (!form.valid) return;
      request<RegisterResponse>('/user/register', {
        method: 'POST',
        body: {
          username: form.data.username,
          password: form.data.password,
          email: form.data.email,
        },
      })
        .then(res => {
          username = res.data.username
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
  <Form.Field {form} name="email">
    <Form.Control let:attrs>
      <Form.Label>email</Form.Label>
      <Input {...attrs} type="email" bind:value={$formData.email} placeholder="Enter the email here"/>
    </Form.Control>
    <Form.FieldErrors />
  </Form.Field>
  <Form.Field {form} name="username">
   <Form.Control let:attrs>
    <Form.Label>username</Form.Label>
    <Input {...attrs} bind:value={$formData.username} placeholder="Enter the username here"/>
   </Form.Control>
   <Form.FieldErrors />
  </Form.Field>
  <Form.Field {form} name="password">
   <Form.Control let:attrs>
    <Form.Label>password</Form.Label>
    <Input {...attrs} type="password" bind:value={$formData.password} placeholder="Enter the password here"/>
   </Form.Control>
   <Form.FieldErrors />
  </Form.Field>
  <Form.Field {form} name="confirmPassword">
   <Form.Control let:attrs>
    <Form.Label>confirmPassword</Form.Label>
    <Input {...attrs} type="password" bind:value={$formData.confirmPassword} placeholder="Enter the confirmPassword here"/>
   </Form.Control>
   <Form.FieldErrors />
  </Form.Field>
  <div class="flex justify-end items-center gap-x-5 mt-3">
    <a
      href="/account"
      rel="noopener"
      class="text-sm text-blue-500 hover:text-blue-700"
    >
      Cancel
    </a>
    <Form.Button>Register</Form.Button>
  </div>
 </form>

 <StatusDialog 
  bind:open={dialogOpen} 
  {status}
 >
 <div class="text-center">
    {#if status === Status.Success}
    <div>
      <span class="text-lg text-black">Register Success</span>
      <br />
      <br />
      <span>Welcome,</span>
      <strong 
        class="
          text-xl 
          underline
        "
      >
        { !!username && 
          username.slice(0, 1).toUpperCase() + username.slice(1)
        }
      </strong>
      <br />
      <br />
      <Button 
        variant="outline" 
        on:click={() => goto('/login')}
      >
        Login
      </Button>
      
    </div>
    {:else if status = Status.Error}
      <span>Register Failed</span>
    {:else}
      <span />
    {/if}
 </div>
</StatusDialog>