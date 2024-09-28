<script lang="ts">
  import * as Table from "$lib/components/ui/table/index.js";
  import { Button } from "$lib/components/ui/button";
  import { Copy } from '$lib/components/ui/copy'

  type Url = {
    url: string;
    shortUrl: string;
  }
  export let urls: Url[] = [];
 </script>
  
 <Table.Root>
  <Table.Caption>A list of your short urls.</Table.Caption>
  <Table.Header>
   <Table.Row>
    <Table.Head class="w-[100px]">Url</Table.Head>
    <Table.Head>Short Url</Table.Head>
    <Table.Head>Action</Table.Head>
   </Table.Row>
  </Table.Header>
  <Table.Body>
   {#each urls as url, i (i)}
    <Table.Row>
     <Table.Cell class="font-medium">{url.url}</Table.Cell>
     <Table.Cell>{url.shortUrl}</Table.Cell>
     <Table.Cell>
      <Button variant="outline">
        <Copy text={url.shortUrl} />
      </Button>
      <Button variant="destructive">
        Delete
      </Button>
     </Table.Cell>
    </Table.Row>
   {/each}
  </Table.Body>
 </Table.Root>