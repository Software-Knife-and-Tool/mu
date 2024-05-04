;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; sequences
;;;
(lib:intern :prelude "sequencep" (:lambda (t) (:if (prelude:listp t) :t (prelude:vectorp t))))

(lib:intern :prelude "length"
   (:lambda (seq)
      (:if (prelude:sequencep seq)
           (:if (prelude:listp seq)
                (lib:length seq)
                (lib:vector-len seq))
           (prelude:raise seq 'prelude:length "not a sequence"))))

(lib:intern :prelude "elt"
   (:lambda (seq nth)
      (:if (prelude:sequencep seq)
           (:if (prelude:fixnump nth)
                (:if (prelude:listp seq)
                     (lib:nth nth seq)
                     (lib:vector-ref seq nth))
                (prelude:raise seq 'prelude:elt "not a sequence"))
           (prelude:raise nth 'prelude:elt "not a fixnum"))))
