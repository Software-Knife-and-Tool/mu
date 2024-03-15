;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; sequences
;;;
(mu:intern :prelude "sequencep" (:lambda (t) (:if (prelude:listp t) :t (prelude:vectorp t))))

(mu:intern :prelude "length"
   (:lambda (seq)
      (:if (prelude:sequencep seq)
           (:if (prelude:listp seq)
                (mu:length seq)
                (mu:sv-len seq))
           (prelude:raise seq 'prelude:length "not a sequence"))))

(mu:intern :prelude "elt"
   (:lambda (seq nth)
      (:if (prelude:sequencep seq)
           (:if (prelude:fixnump nth)
                (:if (prelude:listp seq)
                     (mu:nth nth seq)
                     (mu:sv-ref seq nth))
                (prelude:raise seq 'prelude:elt "not a sequence"))
           (prelude:raise nth 'prelude:elt "not a fixnum"))))
