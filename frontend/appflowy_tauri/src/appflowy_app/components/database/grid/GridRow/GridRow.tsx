import { Virtualizer } from '@tanstack/react-virtual';
import { FC } from 'react';
import { RenderRow, RenderRowType } from './constants';
import { GridCellRow } from './GridCellRow';
import { GridFieldRow } from './GridFieldRow';
import { GridNewRow } from './GridNewRow';
import { GridCalculateRow } from './GridCalculateRow';

export interface GridRowProps {
  row: RenderRow;
  virtualizer: Virtualizer<Element, Element>;
}

export const GridRow: FC<GridRowProps> = ({
  row,
  virtualizer,
}) => {

  switch (row.type) {
    case RenderRowType.Row:
      return (
        <GridCellRow
          rowMeta={row.data.meta}
          virtualizer={virtualizer}
        />
      );
    case RenderRowType.Fields:
      return <GridFieldRow virtualizer={virtualizer} />;
    case RenderRowType.NewRow:
      return <GridNewRow startRowId={row.data.startRowId} groupId={row.data.groupId} />;
    case RenderRowType.Calculate:
      return <GridCalculateRow />;
    default:
      return null;
  }
};
