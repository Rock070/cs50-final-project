<script lang="ts">
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  export let title: string = 'title';
  export let description: string = 'description';
  export let open: boolean = false;
 </script>
  
 <AlertDialog.Root 
  open={open} 
 >
  <AlertDialog.Content>
   <AlertDialog.Header>
    <AlertDialog.Title>{title}</AlertDialog.Title>
    <AlertDialog.Description>
     {description}
    </AlertDialog.Description>
   </AlertDialog.Header>
   <AlertDialog.Footer> 
     <AlertDialog.Cancel>Close</AlertDialog.Cancel>
    </AlertDialog.Footer>
  </AlertDialog.Content>
 </AlertDialog.Root>