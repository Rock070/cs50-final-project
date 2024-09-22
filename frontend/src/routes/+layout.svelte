<script>
	import { page } from '$app/stores'
	import '../app.css';
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
			p-2 md:p-5
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
						class:active-link={$page.url.pathname === '/'}
					>
						home
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
						href="/login"
						rel="noopener"
						class:active-link={$page.url.pathname === '/login'}
					>
						Login
					</a>
				</li>
				<li>
					<a
						href="/register"
						rel="noopener"
						class:active-link={$page.url.pathname === '/register'}
					>
						Register
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