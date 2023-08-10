<script lang="ts" context="module">
  export type AvatarSize = 'sm' | 'md' | 'lg' | 'xl';
</script>

<script lang="ts">
  import classNames from 'classnames';

  import type { CurrentUserFragment } from '$lib/graphql/schema';
  import { clickOutside } from '$lib/actions/click-outside';
  import { page } from '$app/stores';

  export let user: CurrentUserFragment = $page.data.user;
  export let size: AvatarSize = 'md';
  export let dropdown: true | false = false;

  let customClassName: string | null = null;
  export { customClassName as class };

  let isDropdownOpen = false;

  const userNameCharAtZero = user.name.charAt(0).toUpperCase();
  const sizeClassNames = classNames({
    'text-md h-[44px] w-[44px]': size === 'sm',
    'text-lg h-[50px] w-[50px]': size === 'md',
    'text-xl h-[60px] w-[60px]': size === 'lg',
    'text-2xl h-[75px] w-[75px]': size === 'xl',
  });

  const initialsClassNames = classNames(
    'text-white flex items-center justify-center',
    sizeClassNames,
    {
      'bg-slate-400': userNameCharAtZero === 'A' || userNameCharAtZero === 'W',
      'bg-gray-400': userNameCharAtZero === 'B' || userNameCharAtZero === 'X',
      'bg-zinc-400': userNameCharAtZero === 'C' || userNameCharAtZero === 'Y',
      'bg-neutral-400':
        userNameCharAtZero === 'D' || userNameCharAtZero === 'Z',
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

  const handleDropdownClick = () => {
    isDropdownOpen = !isDropdownOpen;
  };
</script>

<button type="button" on:click={handleDropdownClick}>
  <figure class={containerClassNames}>
    <span class={initialsClassNames}
      >{user.name.charAt(0)}{user.surname.charAt(0)}</span
    >
  </figure>
</button>
{#if isDropdownOpen && dropdown}
  <div class="fixed top-12">
    <div
      use:clickOutside
      on:clickOutside={handleDropdownClick}
      id="dropdown"
      class="z-[100] relative bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600"
    >
      <div class="px-4 py-3 text-sm text-gray-900 dark:text-white">
        <div class="truncate">
          {user.name + ' ' + user.surname}
        </div>
        <div class="font-medium truncate">{user.username}</div>
      </div>
      <div class="py-2">
        <a
          href="/logout"
          class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
          >Sign out</a
        >
      </div>
    </div>
  </div>
{/if}
