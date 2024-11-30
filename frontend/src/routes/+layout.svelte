<script lang="ts">
	import { page } from '$app/stores'
	import '../app.css';
	import Icon from '@iconify/svelte'
	import Cookie from 'js-cookie'
	import { userStore } from '$lib/store/user'
	import { request } from "$lib/request";
	import type { LoginResponse } from '$lib/type/api'
	import { COOKIE_KEY } from '$lib/constant'

	export let data: { token: string } = { token: Cookie.get(COOKIE_KEY.TOKEN) || '' };

	if (data.token) {
		request<LoginResponse>('/user', {
			token: data.token,
		})
			.then((res) => {
				userStore.set({
					...res.data,
					token: data.token
				})
			})
			.catch((err) => {
				console.error(err)
			})
	}

	setTimeout(() => {
			console.log("🚀 ~ userStore.update ~ userStore:", $userStore)
		}, 3000)
</script>

<div 
	class="
		max-w-[1440px]
		w-full
		mx-auto
	"
>
	<header
		class="
			h-[10dvh]
			p-3 md:p-8
		"
	>
		<nav 
			class="
				flex
				justify-between
			"
		>
			<ul>
				<li>
					<a 
						href="/"
						rel="noopener"
						class="font-bold flex items-center gap-x-1"
					>
					<div class="w-6 h-6">
						<Icon 
							icon="arcticons:url-checker"
							class="
								text-black-700
								text-2xl
							"
						/>
					</div>
						Shor
					</a>
				</li>
			</ul>
			<ul
				class="
					flex justify-around 
					gap-x-2.5
				"
			>
				<li>
					<a
						href="/account"
						rel="noopener"
						class:active-link={$page.url.pathname === '/account'}
					>
					
					<div class="w-6 h-6">
						<Icon 
							icon="icon-park-outline:people"
							class="
								text-black-700
								text-2xl
							"
							/>
						</div>
					</a>
				</li>
			</ul>
		</nav>
	</header>
		<main
			class="
				h-[85dvh]
				overflow-x-auto
				overflow-y-auto
				p-2 md:p-5
				my-auto
			"
		>
			<slot></slot>
		</main>
	<footer
		class="
			h-[5dvh]
			px-3 md:px-5
			py-2 md:py-2
			flex items-center
		"
	>
		<div
			class="
				text-right
				w-full
				text-black-700
			"
		>
			Copyright Rock070 All rights reserved
		</div>
	</footer>
</div>

<style>
	.active-link {
		@apply 
			text-orange-400
			font-bold
			underline underline-offset-2
		;
	}
</style>