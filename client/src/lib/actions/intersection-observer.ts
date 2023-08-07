export default function intersectionObserver({
  fetch,
  element,
}: { fetch: Function; element: Element }) {
  if (element) {
    const observer = new IntersectionObserver(
      (entries) => {
        const first = entries[0];
        if (first.isIntersecting) {
          fetch();
        }
      },
      { threshold: 1 },
    );
    observer.observe(element);
  }
}
