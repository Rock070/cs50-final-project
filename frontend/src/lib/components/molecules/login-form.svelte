<script lang="ts">
  import * as Form from "$lib/components/ui/form";
  import { userStore } from "$lib/store/user";
  import { COOKIE_KEY } from "$lib/constant";
  import Cookie from "js-cookie";
  import { Status } from '$lib/type'
  import { goto } from '$app/navigation';
  import type { LoginResponse } from '$lib/type/api'
  import { Input } from "$lib/components/ui/input";
  import StatusDialog from "./status-dialog.svelte";
  import { request } from "$lib/request";
  import { loginFormSchema, type LoginFormSchema } from "../../../routes/schema";
  import {
   type SuperValidated,
   type Infer,
   superForm,
  } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  
  export let data: SuperValidated<Infer<LoginFormSchema>>;
  
  let dialogOpen = false
  let status = Status.Success
  let errorMessage = ''

  const form = superForm(data, {
   validators: zodClient(loginFormSchema),
   SPA: true,
   clearOnSubmit: 'errors',
    onUpdate({ form }){
      if (!form.valid) return;
      request<LoginResponse>('/user/login', {
        method: 'POST',
        body: {
          username: form.data.username,
          password: form.data.password,
        },
      })
        .then(res => {
          userStore.set(res.data)
          status = Status.Success
          Cookie.set(COOKIE_KEY.TOKEN, res.data.token)
          goto('/account')
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
  <div class="flex justify-end items-center gap-x-5 mt-3">
    <a
      href="/register"
      rel="noopener"
      class="text-sm text-blue-500 hover:text-blue-700"
    >
      Register
    </a>
    <Form.Button>Login</Form.Button>
  </div>
 </form>

 <StatusDialog 
  bind:open={dialogOpen} 
  {status}
 >
 <div class="text-center">
    {#if status === Status.Success}
    <div>
      <span class="text-lg text-black">Login Success</span>
      <br />
      <br />
      <span>Welcome,</span>
      <strong 
        class="
          text-xl 
          underline
        "
      >
        { !!$userStore?.username && 
          $userStore.username.slice(0, 1).toUpperCase() + $userStore.username.slice(1)
        }
      </strong>
    </div>
    {:else if status = Status.Error}
      <span>Login Failed</span>
    {:else}
      <span />
    {/if}
 </div>
</StatusDialog>