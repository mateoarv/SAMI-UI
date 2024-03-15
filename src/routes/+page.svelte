<script>
	import { Tabs, TabItem, Button, Label, Select, Modal, Toast, Alert } from 'flowbite-svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { ExclamationCircleSolid } from 'flowbite-svelte-icons';

	let selected_port;
	let ports = [];

	let open_modal = false;
	let open_err_toast = false;

	function connection_btn() {
		invoke('get_ports').then((_ports) => {
			console.log(_ports);
			ports = [];
			for (let p of _ports) {
				ports.push({
					value: p,
					name: p
				});
			}
			if (ports.length > 0) {
				selected_port = ports[0].value;
			}
			open_err_toast = false;
			open_modal = true;
		});
	}

	function connect_btn() {
		invoke('connect', { port: selected_port }).then((status) => {
			if (status) {
				open_modal = false;
			} else {
				open_err_toast = true;
			}
		});
	}
</script>

<Tabs style="underline">
	<TabItem open>
		<div slot="title" class="flex items-center gap-2">
			<!-- <UserCircleSolid size="sm" /> -->
			Conexión
		</div>
		<Button on:click={connection_btn}>Conexión</Button>
	</TabItem>
</Tabs>
<Modal title="Conexión" bind:open={open_modal} size="xs">
	<Label>
		Puerto
		<Select size="sm" class="mt-2" items={ports} bind:value={selected_port} placeholder="" />
	</Label>
	<Toast bind:open={open_err_toast} color="red" dismissable={false}>
		<svelte:fragment slot="icon">
			<ExclamationCircleSolid class="h-5 w-5" />
		</svelte:fragment>
		Conexión fallida
	</Toast>
	<div class="text-center">
		<Button on:click={connect_btn} class="hover:animate-err-bounce">Conectarse</Button>
	</div>
</Modal>
