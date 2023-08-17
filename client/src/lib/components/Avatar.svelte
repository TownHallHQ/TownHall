<script lang="ts" context="module">
  export type AvatarSize = 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl';
</script>

<script lang="ts">
  import classNames from 'classnames';

  import { page } from '$app/stores';

  import type { CurrentUserFragment } from '$lib/graphql/schema';
  import { goto } from '$app/navigation';

  export let user: CurrentUserFragment = $page.data.user;
  export let size: AvatarSize = 'md';
  export let redirect: true | false = false;

  let customClassName: string | null = null;
  export { customClassName as class };

  const userNameCharAtZero = user.name.charAt(0).toUpperCase();
  const sizeClassNames = classNames({
    'text-md h-[44px] w-[44px]': size === 'sm',
    'text-lg h-[50px] w-[50px]': size === 'md',
    'text-xl h-[60px] w-[60px]': size === 'lg',
    'text-2xl h-[75px] w-[75px]': size === 'xl',
    'text-3xl h-[90px] w-[90px]': size === '2xl',
    'text-4xl h-[110px] w-[110px]': size === '3xl',
  });

  const initialsClassNames = classNames(
    'text-white flex items-center justify-center',
    sizeClassNames,
    {
      'bg-slate-400': userNameCharAtZero === 'A' || userNameCharAtZero === 'W',
      'bg-gray-400': userNameCharAtZero === 'B' || userNameCharAtZero === 'X',
      'bg-slate-300': userNameCharAtZero === 'C' || userNameCharAtZero === 'Y',
      'bg-slate-500': userNameCharAtZero === 'D' || userNameCharAtZero === 'Z',
      'bg-stone-400': userNameCharAtZero === 'E',
      'bg-red-400': userNameCharAtZero === 'F',
      'bg-orange-400': userNameCharAtZero === 'G',
      'bg-amber-400': userNameCharAtZero === 'H',
      'bg-yellow-400': userNameCharAtZero === 'I',
      'bg-lime-400': userNameCharAtZero === 'J',
      'bg-green-400': userNameCharAtZero === 'K',
      'bg-emerald-400': userNameCharAtZero === 'L',
      'bg-teal-400': userNameCharAtZero === 'M',
      'bg-cyan-400': userNameCharAtZero === 'N',
      'bg-sky-400': userNameCharAtZero === 'O',
      'bg-blue-400': userNameCharAtZero === 'P',
      'bg-indigo-400': userNameCharAtZero === 'Q',
      'bg-violet-400': userNameCharAtZero === 'R',
      'bg-purple-400': userNameCharAtZero === 'S',
      'bg-fuchsia-400': userNameCharAtZero === 'T',
      'bg-pink-400': userNameCharAtZero === 'U',
      'bg-rose-400': userNameCharAtZero === 'V',
    }
  );

  const containerClassNames = classNames(
    'rounded-lg overflow-hidden',
    sizeClassNames,
    customClassName
  );

  const handleClick = () => {
    if (redirect) {
      goto(`/${user.username}`);
    }
  };
</script>

<button type="button" on:click={handleClick}>
  <figure class={containerClassNames}>
    <span class={initialsClassNames}
      >{user.name.charAt(0)}{user.surname.charAt(0)}</span
    >
  </figure>
</button>
