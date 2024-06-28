<script context="module" lang="ts">
	export type Commit = {
		message: string;
		author: string;
		date: Date;
	};
</script>

<script lang="ts">
	export let commit: Commit;
</script>

<div class="flex">
	<div class="w-10">
		<svg version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 25">
			<path d="M 10 0 S 10 0, 10 25" class="fill-none stroke-blue-400" />
			<rect x="7.5" y="10" width="5" height="5" rx="1" class="fill-blue-400" />
		</svg>
	</div>
	<div class="self-center">
		{commit.message}
		<div class="italic text-foreground-dim">
			{commit.author}
		</div>
	</div>
</div>
