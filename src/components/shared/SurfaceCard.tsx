import type { PropsWithChildren, ReactNode } from "react";

type SurfaceCardProps = PropsWithChildren<{
  title: string;
  description: string;
  actions?: ReactNode;
}>;

export function SurfaceCard({
  title,
  description,
  actions,
  children,
}: SurfaceCardProps) {
  return (
    <section className="surface-card">
      <div className="surface-card-header">
        <div>
          <h3>{title}</h3>
          <p>{description}</p>
        </div>
        {actions ? <div className="surface-card-actions">{actions}</div> : null}
      </div>
      {children}
    </section>
  );
}
