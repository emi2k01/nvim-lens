import Link from "next/link";
import { ColorschemeWithPreview } from "@/types";
import { getColorschemesWithPreview } from "@/util/config";

type HomeProps = {
  colorschemesWithPreview: ColorschemeWithPreview[];
};

export async function getStaticProps() {
  const colorschemesWithPreview = await getColorschemesWithPreview();
  return {
    props: {
      colorschemesWithPreview,
    },
  };
}

export default function Home(props: HomeProps) {
  return (
    <div className="flex flex-wrap gap-4 py-6">
      {props.colorschemesWithPreview.map((cs, i) => (
        <Link
          href={{ pathname: cs.plugin.id, query: { colorscheme: cs.name } }}
          key={i}
        >
          <div className="text-[0.25rem] rounded-md overflow-hidden">
            <div
              dangerouslySetInnerHTML={{ __html: cs.previewHtml }}
              className="hover:scale-[1.25] transition-all"
            />
          </div>
          <p className="text-slate-500 text-sm mt-2 flex items-center justify-between">
            <span className="font-medium">{cs.name}</span>
            <span className="text-xs">{cs.plugin.name}</span>
          </p>
        </Link>
      ))}
    </div>
  );
}
