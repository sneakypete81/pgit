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

<div class="p-2">
	{commit.message}
	<div class="italic text-foreground-dim">
		{commit.author}
	</div>
</div>
