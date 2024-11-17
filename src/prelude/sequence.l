;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; sequences
;;;
(mu:intern prelude "sequencep" (:lambda (t) (:if (core:listp t) :t (core:vectorp t))))

(mu:intern prelude "length"
   (:lambda (seq)
      (:if (prelude:sequencep seq)
           (:if (core:listp seq)
                (mu:length seq)
                (mu:vector-len seq))
           (core:raise seq 'prelude:length "not a sequence"))))

(mu:intern prelude "elt"
   (:lambda (seq nth)
      (:if (prelude:sequencep seq)
           (:if (core:fixnump nth)
                (:if (core:listp seq)
                     (mu:nth nth seq)
                     (mu:vector-ref seq nth))
                (core:raise seq 'prelude:elt "not a sequence"))
           (core:raise nth 'prelude:elt "not a fixnum"))))
