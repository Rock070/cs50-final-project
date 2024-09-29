<script lang="ts">
  import * as Table from "$lib/components/ui/table/index.js";
  import { Button } from "$lib/components/ui/button";
  import { Copy } from '$lib/components/ui/copy'
  import { format } from 'date-fns'
  import { userStore } from '$lib/store/user'
  import request from '$lib/request/core'
  import type { UserUrl, UserUrlsResponse } from '$lib/type/api';
  import Alert from "$lib/components/molecules/alert.svelte";
  export let urls: UserUrl[] = [];
  export let open: boolean = false;

  const fetchUrls = () => {
    request<UserUrlsResponse>('/user/urls', {
      token: $userStore.token,
    }).then(res => {
      urls = res.data.urls;
    });
  };

  const deleteUrl = (id: string) => {
    request(`/url`, {
      method: 'DELETE',
      token: $userStore.token,
      body: ({
        id,
      })
    })
    .then(res => {
      // todo 删除成功候跳出 toast 與更新列表
      console.log(res)
      open = true;
      fetchUrls();
    })
  }
 </script>
  
 <Table.Root>
  <Table.Header>
   <Table.Row>
    <Table.Head class="w-[35%]">Url</Table.Head>
    <Table.Head class="w-[35%]">Short Url</Table.Head>
    <Table.Head class="w-[15%]">Created At</Table.Head>
    <Table.Head class="w-[15%]">Action</Table.Head>
   </Table.Row>
  </Table.Header>
  <Table.Body>
   {#each urls as url, i (url.id)}
    <Table.Row>
     <Table.Cell class="font-medium break-all">{url.url}</Table.Cell>
     <Table.Cell>{url.short_url}</Table.Cell>
     <Table.Cell>{format(new Date(url.created_at), 'yyyy-MM-dd HH:mm')}</Table.Cell>
     <Table.Cell>
      <div class="flex items-center gap-x-2">
        <Button variant="outline">
          <Copy text={url.short_url} />
        </Button>
        <Button variant="destructive" on:click={() => deleteUrl(url.id)}>
          Delete
        </Button>
      </div>
     </Table.Cell>
    </Table.Row>
   {/each}
  </Table.Body>
 </Table.Root>

 <Alert open={open} title="success" description="delete success" />